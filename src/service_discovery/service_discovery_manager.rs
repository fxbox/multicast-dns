#[derive(Debug)]
pub struct ServiceDescription<'a> {
    pub address: &'a str,
    pub domain: &'a str,
    pub host_name: &'a str,
    pub interface: i32,
    pub name: &'a str,
    pub port: u16,
    pub type_name: &'a str,
    pub txt: &'a str,
}

pub trait ServiceDiscoveryManager {
    fn new() -> Self;

    // fn discover_services_sync(&self, service_type: &str);
    fn discover_services<F>(&self, service_type: &str, callback: F)
        where F: FnMut(ServiceDescription);
    fn stop_service_discovery(&self);
    fn resolve_service(&self, service_description: ServiceDescription);
}