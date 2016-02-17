#![feature(custom_derive, plugin)]
#![plugin(docopt_macros)]

extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

mod bindings;
mod service_discovery;
use std::sync::mpsc::channel;
use std::thread;

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

fn on_service_resolved(service_description: ServiceDescription) {
    println!("Service resolved: {:?}", service_description);
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let service_discovery_manager: AvahiServiceDiscoveryManager = ServiceDiscoveryManager::new();

    let service_type = args.flag_type.unwrap_or(DEFAULT_SERVICE_TYPE.to_owned());

    // let (tx, rx) = channel::<ServiceDescription>();
//     
//     let mut callback: &mut FnMut(ServiceDescription) = &mut callback;
// 
//     let userdata = unsafe { mem::transmute(&mut callback) };
    

    service_discovery_manager.discover_services(&service_type, |service: ServiceDescription| {
        println!("Service discovered: {:?}", service);

        service_discovery_manager.resolve_service(ServiceDescription {
                                                      address: &"",
                                                      domain: &"local",
                                                      host_name: &"",
                                                      interface: 4,
                                                      name: &"mz-ubuntu",
                                                      port: 0,
                                                      protocol: 0,
                                                      txt: &"",
                                                      type_name: &"_device-info._tcp",
                                                  },
                                                  on_service_resolved);

        // tx.send(service).unwrap();

        // service_discovery_manager.resolve_service(service, on_service_resolved);
    });

    // println!("{:?}", rx.recv().unwrap());

    let mut i: u64 = 0;
    loop {
        //         i = i + 1;
        // 
        //         if i == 10000 {
        //             service_discovery_manager.resolve_service(ServiceDescription {
        //                                                           address: &"",
        //                                                           domain: &"local",
        //                                                           host_name: &"",
        //                                                           interface: 4,
        //                                                           name: &"mz-ubuntu",
        //                                                           port: 0,
        //                                                           protocol: 0,
        //                                                           txt: &"",
        //                                                           type_name: &"_device-info._tcp",
        //                                                       },
        //                                                       on_service_resolved);
        //        }
    }
}