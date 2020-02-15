use adapters::adapter::Adapter;
use adapters::adapter::DiscoveryAdapter;
use adapters::errors::Error;
use adapters::PlatformDependentAdapter;

#[derive(Clone, Copy, Debug)]
pub enum ServiceProtocol {
    IPv4 = 0,
    IPv6 = 1,
    Unspecified = -1,
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
    pub on_service_discovered: Option<&'a dyn Fn(ServiceInfo)>,
    pub on_all_discovered: Option<&'a dyn Fn()>,
}

pub struct ResolveListeners<'a> {
    pub on_service_resolved: Option<&'a dyn Fn(ServiceInfo)>,
}

pub struct DiscoveryManager {
    adapter: Box<dyn DiscoveryAdapter>,
}

impl DiscoveryManager {
    pub fn new() -> Self {
        Default::default()
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

impl Default for DiscoveryManager {
    fn default() -> Self {
        let adapter: Box<dyn DiscoveryAdapter> = Box::new(PlatformDependentAdapter::new());

        DiscoveryManager { adapter }
    }
}
