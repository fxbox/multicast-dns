#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

mod bindings;
mod service_discovery;

use service_discovery::service_discovery_manager::ServiceDescription;
use service_discovery::ServiceDiscoveryManager;
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
    let service_discovery_manager: AvahiServiceDiscoveryManager = ServiceDiscoveryManager::new();

    let service_type = args.flag_type.unwrap_or(DEFAULT_SERVICE_TYPE.to_owned());

    service_discovery_manager.discover_services(&service_type, |service: ServiceDescription| {
        println!("Service discovered: {:?}", service);

        service_discovery_manager.resolve_service(service, |service: ServiceDescription| {
            println!("Service resolved: {:?}", service);
        });
    });

    loop {
    }
}