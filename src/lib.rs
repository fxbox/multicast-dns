extern crate libc;

mod adapters;
#[cfg(target_os = "linux")]
mod bindings;
pub mod discovery;
pub mod host;

use adapters::Adapter;
#[cfg(target_os = "linux")]
use adapters::avahi::AvahiAdapter as PlatformAdapter;
#[cfg(not(target_os = "linux"))]
use adapters::fake::FakeAdapter as PlatformAdapter;

use discovery::DiscoveryManager;
use host::HostManager;

pub struct MulticastDNS {
    pub discovery: DiscoveryManager,
    pub host: HostManager,
}

impl MulticastDNS {
    pub fn new() -> MulticastDNS {
        let discovery_adapter: Box<Adapter> = Box::new(PlatformAdapter::new());
        let host_adapter: Box<Adapter> = Box::new(PlatformAdapter::new());

        MulticastDNS {
            discovery: DiscoveryManager::new(discovery_adapter),
            host: HostManager::new(host_adapter),
        }
    }
}