use adapters::errors::Error;
use discovery::discovery_manager::*;

pub trait DiscoveryAdapter {
    fn start_discovery(
        &self,
        service_type: &str,
        listeners: DiscoveryListeners,
    ) -> Result<(), Error>;
    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners);
    fn stop_discovery(&self);
}

pub trait HostAdapter {
    fn get_name(&self) -> Result<String, Error>;
    fn get_name_fqdn(&self) -> Result<String, Error>;
    fn set_name(&self, host_name: &str) -> Result<String, Error>;
    fn is_valid_name(&self, host_name: &str) -> Result<bool, Error>;
    fn get_alternative_name(&self, host_name: &str) -> Result<String, Error>;
    fn add_name_alias(&self, host_name: &str) -> Result<(), Error>;
}

pub trait Adapter: DiscoveryAdapter + HostAdapter + Drop {
    fn new() -> Self
    where
        Self: Sized;
}
