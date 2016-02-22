#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;
extern crate multicast_dns;

use multicast_dns::MulticastDNS;
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

    let multicast_dns = MulticastDNS::new();

    if args.flag_type.is_some() {
        let on_service_resolved = |service: ServiceInfo| {
            println!("Service resolved: {:?}", service);
        };

        let on_service_discovered = |service: ServiceInfo| {
            println!("Service discovered: {:?}", service);

            let resolve_listeners = ResolveListeners {
                on_service_resolved: Some(&on_service_resolved),
            };

            multicast_dns.discovery.resolve_service(service, resolve_listeners);
        };

        let on_all_discovered = || {
            println!("All services has been discovered");
        };

        let discovery_listeners = DiscoveryListeners {
            on_service_discovered: Some(&on_service_discovered),
            on_all_discovered: Some(&on_all_discovered),
        };

        multicast_dns.discovery.discover_services(&args.flag_type.unwrap(), discovery_listeners);
        multicast_dns.discovery.stop_service_discovery();
    }

    if args.flag_name.is_some() {
        let new_host_name = args.flag_name.unwrap();
        println!("Hostname update ({} -> {}) is requested",
                 multicast_dns.host.get_name(),
                 &new_host_name);
        println!("New Host name: {:?}",
                 multicast_dns.host.set_name(&new_host_name));
    }
}