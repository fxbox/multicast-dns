use adapters::Adapter;

#[derive(Debug)]
pub struct ServiceInfo {
    pub address: Option<String>,
    pub domain: Option<String>,
    pub host_name: Option<String>,
    pub interface: i32,
    pub name: Option<String>,
    pub port: u16,
    pub protocol: i32,
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
    adapter: Box<Adapter>,
}

impl DiscoveryManager {
    pub fn new(adapter: Box<Adapter>) -> DiscoveryManager {
        DiscoveryManager { adapter: adapter }
    }

    pub fn discover_services(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.adapter.start_browser(service_type, listeners);
    }

    pub fn resolve_service(&self, service: ServiceInfo, listeners: ResolveListeners) {
        self.adapter.resolve(service, listeners);
    }

    pub fn stop_service_discovery(&self) {
        self.adapter.stop_browser();
    }

    pub fn get_host_name(&self) -> Option<String> {
        self.adapter.get_host_name()
    }

    pub fn set_host_name(&self, host_name: &str) {
        self.adapter.set_host_name(host_name);
    }

    pub fn is_valid_host_name(&self, host_name: &str) -> bool {
        self.adapter.is_valid_host_name(host_name)
    }

    pub fn get_alternative_host_name(&self, host_name: &str) -> String {
        self.adapter.get_alternative_host_name(host_name)
    }
}