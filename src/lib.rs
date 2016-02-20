extern crate libc;

mod api;
#[cfg(target_os = "linux")]
mod bindings;
mod discovery;

use discovery::DiscoveryManager;

pub struct MulticastDNS {
    discovery: Box<DiscoveryManager>,
}