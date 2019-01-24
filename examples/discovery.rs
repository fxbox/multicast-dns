extern crate multicast_dns;
use multicast_dns::discovery::*;

fn main() {
    let service_type = format!("_device-info._tcp");

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

    discovery_manager.discover_services(&service_type, discovery_listeners).unwrap();
}