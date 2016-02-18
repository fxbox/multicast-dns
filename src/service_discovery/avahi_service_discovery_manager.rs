use service_discovery::service_discovery_manager::*;
use service_discovery::avahi_wrapper::*;

pub struct AvahiServiceDiscoveryManager {
    wrapper: AvahiWrapper,
}

impl ServiceDiscoveryManager for AvahiServiceDiscoveryManager {
    fn new() -> AvahiServiceDiscoveryManager {
        AvahiServiceDiscoveryManager { wrapper: AvahiWrapper::new() }
    }

    fn discover_services<T: DiscoveryListener>(&self, service_type: &str, listener: T) {
        self.wrapper.start_browser(service_type, listener);
    }

    fn resolve_service<T: ResolveListener>(&self, service: ServiceDescription, listener: T) {
        self.wrapper.resolve(service, listener);
    }

    fn stop_service_discovery(&self) {
        panic!("Not");//self.wrapper.stop_browser();
    }
}
