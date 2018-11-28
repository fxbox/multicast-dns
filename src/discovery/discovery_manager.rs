use adapters::adapter::Adapter;
use adapters::adapter::DiscoveryAdapter;
use adapters::errors::Error;
use adapters::PlatformDependentAdapter;

#[derive(Debug)]
pub enum ServiceProtocol {
    IPv4 = 0,
    IPv6 = 1,
    Uspecified = -1,
}

#[derive(Debug)]
pub struct ServiceInfo {
    pub address: Option<String>,
    pub domain: Option<String>,
    pub host_name: Option<String>,
    pub interface: i32,
    pub name: Option<String>,
    pub port: u16,
    pub protocol: ServiceProtocol,
    pub type_name: Option<String>,
    pub txt: Option<String>,
}

pub struct DiscoveryListeners<'a> {
    pub on_service_discovered: Option<&'a Fn(ServiceInfo)>,
    pub on_all_discovered: Option<&'a Fn()>,
}

pub struct ResolveListeners<'a> {
    pub on_service_resolved: Option<&'a Fn(ServiceInfo)>,
}

pub struct DiscoveryManager {
    adapter: Box<DiscoveryAdapter>,
}

impl DiscoveryManager {
    pub fn new() -> DiscoveryManager {
        let adapter: Box<DiscoveryAdapter> = Box::new(PlatformDependentAdapter::new());

        DiscoveryManager { adapter: adapter }
    }

    pub fn discover_services(
        &self,
        service_type: &str,
        listeners: DiscoveryListeners,
    ) -> Result<(), Error> {
        self.adapter.start_discovery(service_type, listeners)
    }

    pub fn resolve_service(&self, service: ServiceInfo, listeners: ResolveListeners) {
        self.adapter.resolve(service, listeners);
    }

    pub fn stop_service_discovery(&self) {
        self.adapter.stop_discovery();
    }
}
