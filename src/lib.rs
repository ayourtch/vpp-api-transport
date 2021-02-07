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

pub trait VppApiTransport {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32;
    fn disconnect(&mut self);

    fn get_msg_index(&mut self, name: &str) -> u16;
    fn get_table_max_index(&mut self) -> u16;

    fn ping(&mut self) -> bool;
    fn dump(&self);
}
