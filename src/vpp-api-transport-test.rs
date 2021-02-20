use std::io::Read;
use vpp_api_transport::*;

fn test_transport(t: &mut vpp_api_transport::VppApiTransport) {
    println!("Connect result: {}", t.connect("api-test", None, 32));
    println!("ping 1");
    t.control_ping();
    t.control_ping();
    t.control_ping();
    std::thread::sleep(std::time::Duration::from_secs(1));
    t.dump();
}

fn main() {
    println!("hi!");
    let mut t1 = shmem::Transport::new();
    let mut t2 = afunix::Transport::new("/tmp/vpp-api.sock");
    test_transport(&mut t1);
    test_transport(&mut t2);
    std::thread::sleep(std::time::Duration::from_secs(60));
    println!("Disconnecting");
    t1.disconnect();
    t2.disconnect();
}
