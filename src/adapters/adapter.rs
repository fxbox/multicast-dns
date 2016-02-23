use discovery::discovery_manager::*;

pub trait DiscoveryAdapter {
    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners);
    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners);
    fn stop_browser(&self);
}

pub trait HostAdapter {
    fn get_host_name(&self) -> String;
    fn set_host_name(&self, host_name: &str) -> String;
    fn is_valid_host_name(&self, host_name: &str) -> bool;
    fn get_alternative_host_name(&self, host_name: &str) -> String;
}

pub trait Adapter : DiscoveryAdapter + HostAdapter {
    fn new() -> Self where Self: Sized;
}