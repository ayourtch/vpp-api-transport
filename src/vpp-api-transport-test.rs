use vpp_api_transport::shmem::*;
use vpp_api_transport::VppApiTransport;

fn main() {
    println!("hi!");
    let mut t = Transport::new();
    println!("Connect result: {}", t.connect("api-test", None, 32));
    let mut t2 = Transport::new();
    println!("Connect result: {}", t2.connect("api-test-2", None, 32));
    println!("ping 1");
    t.ping();
    println!("ping 2");
    t2.ping();
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Disconnecting 1");
    t.disconnect();
    println!("Disconnecting 1");
    t2.disconnect();
}
