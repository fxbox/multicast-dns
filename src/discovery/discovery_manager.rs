#[derive(Debug)]
pub struct ServiceInfo {
    pub address: Option<String>,
    pub domain: Option<String>,
    pub host_name: Option<String>,
    pub interface: i32,
    pub name: Option<String>,
    pub port: u16,
    pub protocol: i32,
    pub type_name: Option<String>,
    pub txt: Option<String>,
}

pub struct DiscoveryListeners<'a> {
    pub on_service_discovered: Option<&'a Fn(ServiceInfo)>,
    pub on_all_discovered: Option<&'a Fn()>,
}

pub struct ResolveListeners<'a> {
    pub on_service_resolved: Option<&'a Fn(ServiceInfo)>,
}

pub trait DiscoveryManager {
    fn new() -> Self where Self: Sized;

    fn discover_services(&self, service_type: &str, listeners: DiscoveryListeners);
    fn stop_service_discovery(&self);
    fn resolve_service(&self, service: ServiceInfo, listeners: ResolveListeners);

    fn get_host_name(&self) -> Option<String>;
    fn set_host_name(&self, host_name: &str);
    fn is_valid_host_name(&self, host_name: &str) -> bool;
    fn get_alternative_host_name(&self, host_name: &str) -> String;
}