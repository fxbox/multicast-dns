#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;
extern crate multicast_dns;

use multicast_dns::host::HostManager;
use multicast_dns::discovery::discovery_manager::*;

docopt!(Args derive Debug, "
Usage: multicast_dns-bin [-t <type>] [-n <hostname>]

Options:
    -t, --type <type>       Look for service of the specified type (e.g. _device-info._tcp).
    -n, --name <hostname>   Set custom host hanme.
",
        flag_type: Option<String>,
        flag_name: Option<String>);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    if args.flag_type.is_some() {
        let discovery_manager = DiscoveryManager::new();

        let on_service_resolved = |service: ServiceInfo| {
            println!("Service resolved: {:?}", service);
        };

        let on_service_discovered = |service: ServiceInfo| {
            println!("Service discovered: {:?}", service);

            let resolve_listeners = ResolveListeners {
                on_service_resolved: Some(&on_service_resolved),
            };

            discovery_manager.resolve_service(service, resolve_listeners);
        };

        let on_all_discovered = || {
            println!("All services has been discovered");
        };

        let discovery_listeners = DiscoveryListeners {
            on_service_discovered: Some(&on_service_discovered),
            on_all_discovered: Some(&on_all_discovered),
        };

        discovery_manager.discover_services(&args.flag_type.unwrap(), discovery_listeners);
    }

    if args.flag_name.is_some() {
        let host_manager = HostManager::new();
        let new_host_name = args.flag_name.unwrap();

        println!("Hostname update ({} -> {}) is requested",
                 host_manager.get_name(),
                 &new_host_name);
        println!("New Host name: {:?}", host_manager.set_name(&new_host_name));
    }
}