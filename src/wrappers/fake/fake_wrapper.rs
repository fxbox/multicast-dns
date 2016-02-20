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
                address: None,
                domain: Some(format!("local")),
                host_name: None,
                interface: 1,
                name: Some(format!("fake")),
                port: 0,
                protocol: 3,
                txt: None,
                type_name: Some(service_type.to_string()),
            });
        }

        if listeners.on_all_discovered.is_some() {
            (*listeners.on_all_discovered.unwrap())();
        }
    }

    fn resolve(&self, service: ServiceDescription, listeners: ResolveListeners) {
        let service = ServiceDescription {
            address: Some(format!("192.168.1.1")),
            domain: service.domain,
            host_name: Some(format!("fake.local")),
            interface: service.interface,
            name: service.name,
            port: 80,
            protocol: service.protocol,
            txt: Some(format!("\"model=Xserve\"")),
            type_name: service.type_name,
        };

        if listeners.on_service_resolved.is_some() {
            (*listeners.on_service_resolved.unwrap())(service);
        }
    }

    fn stop_browser(&self) {}

    fn get_host_name(&self) -> Option<String> {
        return None;
    }

    fn set_host_name(&self, host_name: &str) {
        println!("Setting host name: {}", host_name);
    }

    fn is_valid_host_name(&self, host_name: &str) -> bool {
        println!("Verifying host name: {}", host_name);
        true
    }

    fn get_alternative_host_name(&self, host_name: &str) -> String {
        format!("{}-2", host_name)
    }
}