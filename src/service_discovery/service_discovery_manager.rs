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

pub trait DiscoveryListener {
    fn on_service_discovered(&self, service: ServiceDescription);
    fn on_all_discovered(&self);
}

pub trait ResolveListener {
    fn on_service_resolved(&self, service: ServiceDescription);
}

pub trait ServiceDiscoveryManager {
    fn new() -> Self;

    fn discover_services<T: DiscoveryListener>(&self, service_type: &str, listener: T);
    fn stop_service_discovery(&self);
    fn resolve_service<T: ResolveListener>(&self, service: ServiceDescription, listener: T);
}