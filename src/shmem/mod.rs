mod shmem_bindgen;
use shmem_bindgen::*;

use crate::VppApiTransport;

#[no_mangle]
pub unsafe extern "C" fn shmem_default_cb(data: *mut u8, len: i32) {
    println!("Got {} bytes of data", len);
}

#[no_mangle]
pub unsafe extern "C" fn vac_callback_async(data: *mut u8, len: i32) {
    println!("async Got {} bytes of data", len);
}

#[no_mangle]
pub unsafe extern "C" fn vac_callback_sync(data: *mut u8, len: i32) {
    println!("sync Got {} bytes of data", len);
}

#[no_mangle]
pub unsafe extern "C" fn vac_error_handler(arg: *mut u8, msg: *mut u8, msg_len: i32) {
    println!("Error: {} bytes of message", msg_len);
}

pub struct Transport {
    connected: bool,
}

impl Transport {
    pub fn new() -> Self {
        unsafe { vac_mem_init(0) };
        Transport { connected: false }
    }
}

impl VppApiTransport for Transport {
    fn connect(&mut self, name: &str, chroot_prefix: Option<&str>, rx_qlen: i32) -> i32 {
        use std::ffi::CString;

        let name_c = CString::new(name).unwrap();
        let chroot_prefix_c = chroot_prefix.map(|x| CString::new(x).unwrap());

        let name_arg = name_c.as_ptr() as *mut i8;
        let chroot_prefix_arg = if let Some(p) = chroot_prefix_c {
            p.as_ptr()
        } else {
            std::ptr::null_mut()
        } as *mut i8;
        let err =
            unsafe { vac_connect(name_arg, chroot_prefix_arg, Some(shmem_default_cb), rx_qlen) };
        if err == 0 {
            self.connected = true;
        }
        return err;
    }
    fn disconnect(&mut self) {
        if self.connected {
            let err = unsafe { vac_disconnect() };
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
        let err = unsafe { vac_write(cstr_mut!("\x02\x4d234556789b123456789c123456789d123"), 32) };
        true
    }
}

pub fn shmem_test() {
    unsafe {
        println!("VAC connecting...");
        vac_mem_init(32000000);
        let err = vac_connect(
            cstr_mut!("test-rust-api"),
            std::ptr::null_mut(),
            Some(shmem_default_cb),
            32,
        );
        println!("connect result: {}", err);
        for i in 1..3 {
            let err = vac_write(cstr_mut!("\x02\x4d234556789b123456789c123456789d123"), 32);
            println!("write result: {}", err);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
