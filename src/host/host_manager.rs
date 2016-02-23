use adapters::adapter::Adapter;
use adapters::adapter::HostAdapter;
use adapters::PlatformDependentAdapter;

pub struct HostManager {
    adapter: Box<HostAdapter>,
}

impl HostManager {
    pub fn new() -> HostManager {
        let adapter: Box<HostAdapter> = Box::new(PlatformDependentAdapter::new());

        HostManager { adapter: adapter }
    }

    pub fn get_name(&self) -> String {
        self.adapter.get_name()
    }

    pub fn set_name(&self, name: &str) -> String {
        self.adapter.set_name(name)
    }

    pub fn is_valid_name(&self, name: &str) -> bool {
        self.adapter.is_valid_name(name)
    }

    pub fn get_alternative_name(&self, name: &str) -> String {
        self.adapter.get_alternative_name(name)
    }
}