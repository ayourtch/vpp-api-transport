use bincode;
use bincode::Options;
use serde::{Deserialize, Serialize};
use std::os::unix::net::UnixStream;

use crate::VppApiTransport;
use std::collections::HashMap;

use crate::get_encoder;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

mod big_array;
use big_array::BigArray;

#[derive(Debug, Default)]
struct GlobalState {
    created: bool,
    receive_buffer: VecDeque<u8>,
}

lazy_static! {
    static ref GLOBAL: Arc<Mutex<GlobalState>> = {
        let mut gs = GlobalState {
            ..Default::default()
        };

        Arc::new(Mutex::new(gs))
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct SockMsgHeader {
    _q: u64,
    msglen: u32,
    gc_mark: u32,
}

pub struct Transport {
    connected: bool,
    sock_path: String,
    sock: Option<std::os::unix::net::UnixStream>,
    message_name_to_id: HashMap<String, u16>,
    message_max_index: u16,
    client_index: u32,
}

impl Transport {
    pub fn new(path: &str) -> Self {
        let mut gs = GLOBAL.lock().unwrap();
        if gs.created {
            panic!("One transport already created!");
        }

        gs.created = true;

        Transport {
            connected: false,
            sock_path: path.to_owned(),
            sock: None,
            message_name_to_id: HashMap::new(),
            message_max_index: 0,
            client_index: 0,
        }
    }
}

impl std::io::Read for Transport {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.sock.as_ref().unwrap().read(buf)
    }
}
impl std::io::Write for Transport {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let hdr = SockMsgHeader {
            _q: 0,
            msglen: buf.len() as u32,
            gc_mark: 0,
        };
        let hdre = get_encoder().serialize(&hdr).unwrap();

        self.sock.as_ref().unwrap().write(&hdre)?;
        self.sock.as_ref().unwrap().write(buf)?;
        self.sock.as_ref().unwrap().flush()?;
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.sock.as_ref().unwrap().flush()
    }
}

type u8_64 = [u8; 64];

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgSockClntCreate {
    _vl_msg_id: u16,
    context: u32,
    #[serde(with = "BigArray")]
    name: u8_64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgSockClntCreateReplyHdr {
    _vl_msg_id: u16,
    client_index: u32,
    context: u32,
    response: i32,
    index: u32,
    count: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgSockClntCreateReplyEntry {
    index: u16,
    #[serde(with = "BigArray")]
    name: u8_64,
}

impl VppApiTransport for Transport {
    fn connect(
        &mut self,
        name: &str,
        chroot_prefix: Option<&str>,
        rx_qlen: i32,
    ) -> std::io::Result<()> {
        use std::io::Write;

        let mut s = UnixStream::connect(&self.sock_path)?;
        self.sock = Some(s);
        self.connected = true;

        /* FIXME: this is ugly and odd, there's gotta be a better way... */
        let mut name1 = name.to_string();
        let mut name_a: [u8; 64] = [0; 64];
        while name1.len() < name_a.len() {
            name1.push('\0');
        }
        name_a.copy_from_slice(&name1.as_bytes());

        let sockclnt_create = MsgSockClntCreate {
            _vl_msg_id: 15,
            context: 124,
            name: name_a,
        };

        let scs = get_encoder().serialize(&sockclnt_create).unwrap();

        let mut s = self.sock.as_ref().unwrap();
        self.write(&scs)?;
        let buf = self.read_one_msg()?;
        let hdr: MsgSockClntCreateReplyHdr = get_encoder().deserialize(&buf[0..20]).unwrap();
        self.client_index = hdr.index as u32;
        let mut i = 0;
        self.message_max_index = hdr.count;
        while i < hdr.count as usize {
            let sz = 66; /* MsgSockClntCreateReplyEntry */
            let ofs1 = 20 + i * 66;
            let ofs2 = ofs1 + sz;

            let msg: MsgSockClntCreateReplyEntry =
                get_encoder().deserialize(&buf[ofs1..ofs2]).unwrap();
            let msg_name_trailing_zero = String::from_utf8_lossy(&msg.name);
            let msg_name = msg_name_trailing_zero.trim_right_matches("\u{0}");
            self.message_name_to_id.insert(msg_name.into(), msg.index);
            i = i + 1;
        }
        Ok(())
    }
    fn disconnect(&mut self) {
        if self.connected {
            self.sock = None;
            self.connected = false;
        }
    }
    fn set_nonblocking(&mut self, nonblocking: bool) -> std::io::Result<()> {
        if let Some(ref mut s) = self.sock {
            s.set_nonblocking(nonblocking)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "trying to set unconnected socket non-blocking",
            ))
        }
    }

    fn get_client_index(&self) -> u32 {
        self.client_index
    }
    fn get_msg_index(&mut self, name: &str) -> Option<u16> {
        self.message_name_to_id.get(name).map(|x| x.to_owned())
    }
    fn get_table_max_index(&mut self) -> u16 {
        0
    }
    fn dump(&self) {
        let mut gs = GLOBAL.lock().unwrap();
        println!("Global state: {:?}", &gs);
    }
}
