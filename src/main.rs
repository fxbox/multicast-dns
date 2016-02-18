#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

mod bindings;
mod service_discovery;

use service_discovery::service_discovery_manager::ServiceDescription;
use service_discovery::service_discovery_manager::DiscoveryListener;
use service_discovery::service_discovery_manager::ResolveListener;
use service_discovery::ServiceDiscoveryManager;
use service_discovery::AvahiServiceDiscoveryManager;

const DEFAULT_SERVICE_TYPE: &'static str = "_device-info._tcp";

docopt!(Args derive Debug, "
Usage: multicast_dns [-t <type>]

Options:
    -t, --type <type>  Look for service of the specified type (default is _device-info._tcp).
",
        flag_type: Option<String>);


struct TestDiscoveryListener<'a> {
    manager: &'a AvahiServiceDiscoveryManager,
}
impl<'a> DiscoveryListener for TestDiscoveryListener<'a> {
    fn on_service_discovered(&self, service: ServiceDescription) {
        println!("Service discovered: {:?}", service);

        self.manager.resolve_service(service, TestResolveListener);
    }

    fn on_all_discovered(&self) {
        println!("All discovered");
    }
}

struct TestResolveListener;
impl ResolveListener for TestResolveListener {
    fn on_service_resolved(&self, service: ServiceDescription) {
        println!("Service resolved: {:?}", service);
    }
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let discovery_manager: AvahiServiceDiscoveryManager = ServiceDiscoveryManager::new();

    let service_type = args.flag_type.unwrap_or(DEFAULT_SERVICE_TYPE.to_owned());

    // let mut first_service: Option<ServiceDescription> = None;

    //     let listener = DiscoveryListener {
    //         on_service_found: &|service: ServiceDescription| {
    //             println!("Service discovered: {:?}", service);
    //         },
    //
    //         on_all_discovered: &|| {
    //             println!("All discovered");
    //         },
    //     };

    discovery_manager.discover_services(&service_type,
                                        TestDiscoveryListener { manager: &discovery_manager });

    loop {
        // if first_service.is_some() {
        //     break;
        // }
    }
}