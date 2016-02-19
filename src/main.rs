#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

#[cfg(target_os = "linux")]
mod bindings;

mod wrappers;
mod service_discovery;

use service_discovery::service_discovery_manager::*;
use service_discovery::AvahiServiceDiscoveryManager;

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

    let discovery_manager: AvahiServiceDiscoveryManager = ServiceDiscoveryManager::new();

    let on_service_resolved = |service: ServiceDescription| {
        println!("Service resolved: {:?}", service);
    };

    let on_service_discovered = |service: ServiceDescription| {
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

    discovery_manager.discover_services(service_type, discovery_listeners);

    loop {}

    // discovery_manager.stop_service_discovery();
}