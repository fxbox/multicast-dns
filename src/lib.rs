extern crate libc;

mod adapters;
#[cfg(target_os = "linux")]
mod bindings;
pub mod discovery;

use adapters::Adapter;
#[cfg(target_os = "linux")]
use adapters::avahi::AvahiAdapter as PlatformAdapter;
#[cfg(not(target_os = "linux"))]
use adapters::fake::FakeAdapter as PlatformAdapter;

use discovery::DiscoveryManager;

pub struct MulticastDNS {
    pub discovery: Box<DiscoveryManager>,
}

impl MulticastDNS {
    pub fn new() -> MulticastDNS {
        let adapter: Box<Adapter> = Box::new(PlatformAdapter::new());

        MulticastDNS { discovery: Box::new(DiscoveryManager::new(adapter)) }
    }
}