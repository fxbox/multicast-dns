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

pub struct DiscoveryListener<'a> {
    pub on_service_found: &'a Fn(ServiceDescription),
    pub on_all_discovered: &'a Fn(),
}

pub trait ServiceDiscoveryManager {
    fn new() -> Self;

    fn discover_services(&self, service_type: &str, listener: DiscoveryListener);
    fn stop_service_discovery(&self);
    fn resolve_service<F>(&self, service_description: ServiceDescription, callback: F)
        where F: Fn(ServiceDescription);
}