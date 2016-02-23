use adapters::adapter::*;
use discovery::discovery_manager::*;

pub struct FakeAdapter;

impl DiscoveryAdapter for FakeAdapter {
    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners) {
        if listeners.on_service_discovered.is_some() {
            (*listeners.on_service_discovered.unwrap())(ServiceInfo {
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

    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners) {
        let service = ServiceInfo {
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
}

impl HostAdapter for FakeAdapter {
    fn get_host_name(&self) -> String {
        return "fake".to_owned();
    }

    fn set_host_name(&self, host_name: &str) -> String {
        host_name.to_owned()
    }

    fn is_valid_host_name(&self, host_name: &str) -> bool {
        println!("Verifying host name: {}", host_name);
        true
    }

    fn get_alternative_host_name(&self, host_name: &str) -> String {
        format!("{}-2", host_name)
    }
}

impl Adapter for FakeAdapter {
    fn new() -> FakeAdapter {
        FakeAdapter
    }
}