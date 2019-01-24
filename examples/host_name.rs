extern crate multicast_dns;
use multicast_dns::host::HostManager;

fn main() {
    let host_name = format!("custom-host");

    let host_manager = HostManager::new();

    if !host_manager.is_valid_name(&host_name).unwrap() {
        panic!("Host name `{}` is not a valid host name!", &host_name);
    }

    // The `new_host_name` can be different from the one we are trying to set,
    // due to possible collisions that may happen.
    let new_host_name = host_manager.set_name(&host_name).unwrap();

    println!("New host name is: {:?}", &new_host_name);
}
