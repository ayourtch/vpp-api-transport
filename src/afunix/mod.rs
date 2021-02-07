use bincode;
use bincode::Options;
use serde::{Deserialize, Serialize};
use std::os::unix::net::UnixStream;

use crate::VppApiTransport;

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

        self.sock.as_ref().unwrap().write(&hdre);
        self.sock.as_ref().unwrap().write(buf);
        self.sock.as_ref().unwrap().flush();
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

impl VppApiTransport for Transport {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32 {
        use std::io::Write;

        let mut sock = UnixStream::connect(&self.sock_path);
        if let Ok(s) = sock {
            self.sock = Some(s);
            println!("Open success!");
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
            self.write(&scs);
            self.read_one_msg();

            return 0;
        }
        eprintln!("Error: {:?}", &sock);
        return -1;
    }
    fn disconnect(&mut self) {
        if self.connected {
            self.sock = None;
            self.connected = false;
        }
    }
    fn get_msg_index(&mut self, name: &str) -> u16 {
        0
    }
    fn get_table_max_index(&mut self) -> u16 {
        0
    }
    fn ping(&mut self) -> bool {
        use std::io::Write;
        self.write(b"\x02\x4d234556789b123456789c123456789d123");
        true
    }
    fn dump(&self) {
        let mut gs = GLOBAL.lock().unwrap();
        println!("Global state: {:?}", &gs);
    }
}
