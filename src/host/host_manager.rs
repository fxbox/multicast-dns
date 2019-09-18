use adapters::adapter::Adapter;
use adapters::adapter::HostAdapter;
use adapters::errors::Error;
use adapters::PlatformDependentAdapter;

pub struct HostManager {
    adapter: Box<HostAdapter>,
}

impl HostManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_name(&self) -> Result<String, Error> {
        self.adapter.get_name()
    }

    pub fn set_name(&self, name: &str) -> Result<String, Error> {
        self.adapter.set_name(name)
    }

    pub fn is_valid_name(&self, name: &str) -> Result<bool, Error> {
        self.adapter.is_valid_name(name)
    }

    pub fn get_alternative_name(&self, name: &str) -> Result<String, Error> {
        self.adapter.get_alternative_name(name)
    }

    pub fn add_name_alias(&self, name: &str) -> Result<(), Error> {
        self.adapter.add_name_alias(name)
    }

    pub fn announce_service(&self, name: &str, service_type: &str, port: u16) -> Result<(), Error> {
        self.adapter.announce_service(name, service_type, port)
    }
}

impl Default for HostManager {
    fn default() -> Self {
        let adapter: Box<HostAdapter> = Box::new(PlatformDependentAdapter::new());

        HostManager { adapter }
    }
}
