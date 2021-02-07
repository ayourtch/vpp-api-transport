use vpp_api_transport::shmem::*;
use vpp_api_transport::VppApiTransport;

fn main() {
    println!("hi!");
    let mut t = Transport::new();
    println!("Connect result: {}", t.connect("api-test", None, 32));
    println!("ping 1");
    t.ping();
    std::thread::sleep(std::time::Duration::from_secs(1));
    t.dump();
    println!("Disconnecting 1");
    t.disconnect();
}
