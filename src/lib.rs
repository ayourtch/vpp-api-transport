#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;
pub mod afunix;
pub mod shmem;
use bincode;
use bincode::Options;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct SockMsgHeader {
    _q: u64,
    msglen: u32,
    gc_mark: u32,
}

fn get_encoder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Error {
    NoDataAvailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawControlPing {
    _vl_msg_id: u16,
    client_index: u32,
    context: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawControlPingReply {
    _vl_msg_id: u16,
    context: u32,
    retval: i32,
    client_index: u32,
    vpe_pid: u32,
}

pub trait VppApiTransport: Read + Write {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32;
    fn disconnect(&mut self);

    fn get_msg_index(&mut self, name: &str) -> u16;
    fn get_table_max_index(&mut self) -> u16;
    fn get_client_index(&mut self) -> u32;

    fn control_ping(&mut self) -> u32 {
        let control_ping_id = self.get_msg_index("control_ping_51077d14");
        use std::io::Write;
        let context = 42; // FIXME: use atomic autoincrementing
        let msg = RawControlPing {
            _vl_msg_id: control_ping_id,
            client_index: self.get_client_index(),
            context,
        };
        let data = get_encoder().serialize(&msg).unwrap();
        self.write(&data);
        context
    }

    fn skip_to_control_ping_reply(&mut self, context: u32) -> Result<(), Error> {
        let control_ping_reply_id = self.get_msg_index("control_ping_reply_f6b0b8ca");
        loop {
            match self.read_one_msg_id_and_msg() {
                Err(e) => return Err(e),
                Ok((msg_id, data)) => {
                    if msg_id == control_ping_reply_id {
                        // FIXME: deserialize and match the context
                        return Ok(());
                    }
                }
            }
        }
    }

    fn dump(&self);

    fn read_one_msg_into(&mut self, data: &mut Vec<u8>) -> Result<(), Error> {
        let mut header_buf = [0; 16];

        self.read(&mut header_buf).unwrap();
        let hdr: SockMsgHeader = get_encoder().deserialize(&header_buf[..]).unwrap();

        let target_size = hdr.msglen as usize;

        data.resize(target_size, 0);
        let mut got = 0;
        while got < target_size {
            let n = self.read(&mut data[got..target_size]).unwrap();
            println!("Got: {}, n: {}, target_size: {}", got, n, target_size);
            got = got + n;
        }
        Ok(())
    }

    fn read_one_msg(&mut self) -> Result<Vec<u8>, Error> {
        let mut out: Vec<u8> = vec![];
        match self.read_one_msg_into(&mut out) {
            Ok(()) => Ok(out),
            Err(e) => Err(e),
        }
    }

    fn read_one_msg_id_and_msg(&mut self) -> Result<(u16, Vec<u8>), Error> {
        match self.read_one_msg() {
            Ok(ret) => {
                if ret.len() == 0 {
                    Err(Error::NoDataAvailable)
                } else {
                    let msg_id: u16 = ((ret[0] as u16) << 8) + (ret[1] as u16);
                    Ok((msg_id, ret[2..].to_vec()))
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::afunix;
    use crate::shmem;
    use crate::VppApiTransport;

    #[test]
    fn test_shmem_connect() {
        let mut t1 = shmem::Transport::new();
        let res = t1.connect("test", None, 32);
        assert_eq!(res, 0);
        t1.disconnect();
        drop(t1);
    }

    #[test]
    fn test_afunix_connect() {
        let mut t1 = afunix::Transport::new("/run/vpp/api.sock");
        let res = t1.connect("test", None, 32);
        assert_eq!(res, 0);
        let context = t1.control_ping();
        let res = t1.skip_to_control_ping_reply(context);
        assert_eq!(res, Ok(()));
        t1.disconnect();
        drop(t1);
    }
}
