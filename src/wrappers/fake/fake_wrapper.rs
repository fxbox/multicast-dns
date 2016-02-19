use wrappers::wrapper::Wrapper;
use service_discovery::service_discovery_manager::*;

pub struct FakeWrapper;

impl Wrapper for FakeWrapper {
    fn new() -> FakeWrapper {
        FakeWrapper
    }

    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners) {
        if listeners.on_service_discovered.is_some() {
            (*listeners.on_service_discovered.unwrap())(ServiceDescription {
                address: &"",
                domain: &"local",
                host_name: &"",
                interface: 1,
                name: &"fake",
                port: 0,
                protocol: 3,
                txt: &"",
                type_name: service_type,
            });
        }

        if listeners.on_all_discovered.is_some() {
            (*listeners.on_all_discovered.unwrap())();
        }
    }

    fn resolve(&self, service: ServiceDescription, listeners: ResolveListeners) {
        let service = ServiceDescription {
            address: &"192.168.1.1",
            domain: service.domain,
            host_name: &"fake.local",
            interface: service.interface,
            name: service.name,
            port: 80,
            protocol: service.protocol,
            txt: &"\"model=Xserve\"",
            type_name: service.type_name,
        };

        if listeners.on_service_resolved.is_some() {
            (*listeners.on_service_resolved.unwrap())(service);
        }
    }

    fn stop_browser(&self) {}
}