use discovery::discovery_manager::*;

pub trait DiscoveryAdapter {
    fn start_discovery(&self, service_type: &str, listeners: DiscoveryListeners);
    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners);
    fn stop_discovery(&self);
}

pub trait HostAdapter {
    fn get_name(&self) -> String;
    fn set_name(&self, host_name: &str) -> String;
    fn is_valid_name(&self, host_name: &str) -> bool;
    fn get_alternative_name(&self, host_name: &str) -> String;
}

pub trait Adapter : DiscoveryAdapter + HostAdapter + Drop {
    fn new() -> Self where Self: Sized;
}