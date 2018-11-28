use adapters::adapter::Adapter;
use adapters::adapter::HostAdapter;
use adapters::errors::Error;
use adapters::PlatformDependentAdapter;

pub struct HostManager {
    adapter: Box<HostAdapter>,
}

impl HostManager {
    pub fn new() -> HostManager {
        let adapter: Box<HostAdapter> = Box::new(PlatformDependentAdapter::new());

        HostManager { adapter: adapter }
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
}
