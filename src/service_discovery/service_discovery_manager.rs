#[derive(Debug)]
pub struct ServiceDescription<'a> {
    pub address: &'a str,
    pub domain: &'a str,
    pub host_name: &'a str,
    pub interface: i32,
    pub name: &'a str,
    pub port: u16,
    pub protocol: i32,
    pub type_name: &'a str,
    pub txt: &'a str,
}

pub struct DiscoveryListeners<'a> {
    pub on_service_discovered: Option<&'a Fn(ServiceDescription)>,
    pub on_all_discovered: Option<&'a Fn()>,
}

pub struct ResolveListeners<'a> {
    pub on_service_resolved: Option<&'a Fn(ServiceDescription)>,
}

pub trait DiscoveryListener {
    fn on_service_discovered(&self, service: ServiceDescription);
    fn on_all_discovered(&self);
}

pub trait ResolveListener {
    fn on_service_resolved(&self, service: ServiceDescription);
}

pub trait ServiceDiscoveryManager {
    fn new() -> Self;

    fn discover_services(&self, service_type: &str, listeners: DiscoveryListeners);
    fn stop_service_discovery(&self);
    fn resolve_service(&self, service: ServiceDescription, listeners: ResolveListeners);
}