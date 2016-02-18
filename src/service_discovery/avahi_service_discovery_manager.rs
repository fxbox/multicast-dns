use service_discovery::service_discovery_manager::*;
use service_discovery::avah_wrapper::*;

pub struct AvahiServiceDiscoveryManager {
    wrapper: AvahiWrapper,
}

impl ServiceDiscoveryManager for AvahiServiceDiscoveryManager {
    fn new() -> AvahiServiceDiscoveryManager {
        AvahiServiceDiscoveryManager { wrapper: AvahiWrapper::new() }
    }

    fn discover_services(&self, service_type: &str, listener: DiscoveryListener) {
        self.wrapper.start_browser(service_type, listener);
    }

    fn resolve_service<F>(&self, service_description: ServiceDescription, callback: F)
        where F: Fn(ServiceDescription)
    {
        self.wrapper.resolve(service_description, callback);
    }

    fn stop_service_discovery(&self) {
        self.wrapper.stop_browser();
    }
}
