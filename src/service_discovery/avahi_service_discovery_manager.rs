use service_discovery::service_discovery_manager::*;
use wrappers::wrapper::Wrapper;

#[cfg(target_os = "linux")]
use wrappers::avahi::AvahiWrapper as APIWrapper;
#[cfg(not(target_os = "linux"))]
use wrappers::fake::FakeWrapper as APIWrapper;

pub struct AvahiServiceDiscoveryManager {
    wrapper: Box<Wrapper>,
}

impl ServiceDiscoveryManager for AvahiServiceDiscoveryManager {
    fn new() -> AvahiServiceDiscoveryManager {
        AvahiServiceDiscoveryManager { wrapper: Box::new(APIWrapper::new()) }
    }

    fn discover_services(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.wrapper.start_browser(service_type, listeners);
    }

    fn resolve_service(&self, service: ServiceDescription, listeners: ResolveListeners) {
        self.wrapper.resolve(service, listeners);
    }

    fn stop_service_discovery(&self) {
        self.wrapper.stop_browser();
    }
}
