#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;
extern crate multicastdnslib;

use multicastdnslib::MulticastDNS;
use multicastdnslib::discovery::discovery_manager::*;

const DEFAULT_SERVICE_TYPE: &'static str = "_device-info._tcp";

docopt!(Args derive Debug, "
Usage: multicast_dns [-t <type>]

Options:
    -t, --type <type>  Look for service of the specified type (default is _device-info._tcp).
",
        flag_type: Option<String>);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let service_type = args.flag_type.unwrap_or(DEFAULT_SERVICE_TYPE.to_owned());

    let multicast_dns = MulticastDNS::new();

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

//     multicast_dns.discovery.discover_services(&service_type, discovery_listeners);
// 
//     println!("Host name: {:?}", multicast_dns.host.get_name());
// 
//     println!("Is valid host name: {:?} - {:?}",
//              format!("foxbox+3"),
//              multicast_dns.host.is_valid_name(&format!("foxbox")));
// 
//     println!("Is valid host name: {:?} - {:?}",
//              format!("foxbox-2"),
//              multicast_dns.host.is_valid_name(&format!("foxbox-2")));
// 
//     println!("Is valid host name: {:?} - {:?}",
//              format!("foxbox.org"),
//              multicast_dns.host.is_valid_name(&format!("foxbox.org")));
// 
//     println!("Alternative to {:?} is {:?}",
//              format!("foxbox"),
//              multicast_dns.host.get_alternative_name(&format!("foxbox")));
// 
//     println!("Alternative to {:?} is {:?}",
//              format!("foxbox-2"),
//              multicast_dns.host.get_alternative_name(&format!("foxbox-2")));
// 
//     println!("Alternative to {:?} is {:?}",
//              format!("foxbox-3"),
//              multicast_dns.host.get_alternative_name(&format!("foxbox-3")));
// 
//     multicast_dns.discovery.stop_service_discovery();
// 
//     println!("Going to loop");

    multicast_dns.host.set_name(&format!("foxbox-4"));

    loop {}
    // discovery_manager.stop_service_discovery();
}