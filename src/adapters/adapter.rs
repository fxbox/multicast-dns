use discovery::discovery_manager::*;

pub trait Adapter {
    fn new() -> Self where Self: Sized;

    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners);

    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners);

    fn stop_browser(&self);

    fn get_host_name(&self) -> Option<String>;
    fn set_host_name(&self, host_name: &str);
    fn is_valid_host_name(&self, host_name: &str) -> bool;
    fn get_alternative_host_name(&self, host_name: &str) -> String;
}