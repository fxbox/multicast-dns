extern crate multicast_dns;
use multicast_dns::host::*;

fn main() {
    let service_type = "_something._tcp";
    let port = 8083;

    let host_manager = HostManager::new();

    host_manager.announce_service("my local service", service_type, port).unwrap();

    println!("Press enter to exit example");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
