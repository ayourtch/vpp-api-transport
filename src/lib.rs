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

pub trait VppApiTransport: std::io::Read {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32;
    fn disconnect(&mut self);

    fn get_msg_index(&mut self, name: &str) -> u16;
    fn get_table_max_index(&mut self) -> u16;

    fn ping(&mut self) -> bool;
    fn dump(&self);

    fn read_one_msg(&mut self) -> Vec<u8> {
        let mut header_buf = [0; 16];

        self.read(&mut header_buf).unwrap();
        let hdr: SockMsgHeader = get_encoder().deserialize(&header_buf[..]).unwrap();

        let target_size = hdr.msglen as usize;

        let mut data: Vec<u8> = vec![];
        while data.len() < target_size {
            let mut buf = [0; 65536];
            let n = self.read(&mut buf).unwrap();
            data.extend_from_slice(&mut buf[0..n]);
        }
        data
    }
}
