use adapters::Adapter;

pub struct HostManager {
    adapter: Box<Adapter>,
}

impl HostManager {
    pub fn new(adapter: Box<Adapter>) -> HostManager {
        HostManager { adapter: adapter }
    }

    pub fn get_name(&self) -> Option<String> {
        self.adapter.get_host_name()
    }

    pub fn set_name(&self, name: &str) {
        self.adapter.set_host_name(name);
    }

    pub fn is_valid_name(&self, name: &str) -> bool {
        self.adapter.is_valid_host_name(name)
    }

    pub fn get_alternative_name(&self, name: &str) -> String {
        self.adapter.get_alternative_host_name(name)
    }
}