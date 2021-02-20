#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
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

pub trait VppApiTransport: Read + Write {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32;
    fn disconnect(&mut self);

    fn get_msg_index(&mut self, name: &str) -> u16;
    fn get_table_max_index(&mut self) -> u16;
    fn get_client_index(&mut self) -> u32;

    fn ping(&mut self) -> bool;
    fn dump(&self);

    fn read_one_msg_into(&mut self, data: &mut Vec<u8>) {
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
    }

    fn read_one_msg(&mut self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        self.read_one_msg_into(&mut out);
        out
    }

    fn read_one_msg_id_and_msg(&mut self) -> (u16, Vec<u8>) {
        let ret = self.read_one_msg();
        let msg_id: u16 = ((ret[0] as u16) << 8) + (ret[1] as u16);
        (msg_id, ret[2..].to_vec())
    }
}
