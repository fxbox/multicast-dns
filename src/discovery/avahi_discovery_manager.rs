use discovery::discovery_manager::*;
use api::api::API;

#[cfg(target_os = "linux")]
use api::avahi::AvahiAPI as DiscoveryAPI;
#[cfg(not(target_os = "linux"))]
use api::fake::FakeAPI as DiscoveryAPI;

pub struct AvahiDiscoveryManager {
    api: Box<API>,
}

impl DiscoveryManager for AvahiDiscoveryManager {
    fn new() -> AvahiDiscoveryManager {
        AvahiDiscoveryManager { api: Box::new(DiscoveryAPI::new()) }
    }

    fn discover_services(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.api.start_browser(service_type, listeners);
    }

    fn resolve_service(&self, service: ServiceInfo, listeners: ResolveListeners) {
        self.api.resolve(service, listeners);
    }

    fn stop_service_discovery(&self) {
        self.api.stop_browser();
    }

    fn get_host_name(&self) -> Option<String> {
        self.api.get_host_name()
    }

    fn set_host_name(&self, host_name: &str) {
        self.api.set_host_name(host_name);
    }

    fn is_valid_host_name(&self, host_name: &str) -> bool {
        self.api.is_valid_host_name(host_name)
    }

    fn get_alternative_host_name(&self, host_name: &str) -> String {
        self.api.get_alternative_host_name(host_name)
    }
}
