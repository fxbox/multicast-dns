use service_discovery::service_discovery_manager::*;

pub trait Wrapper {
    fn new() -> Self where Self: Sized;

    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners);

    fn resolve(&self, service: ServiceDescription, listeners: ResolveListeners);

    fn stop_browser(&self);
}