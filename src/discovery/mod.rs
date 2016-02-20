pub use self::discovery_manager::DiscoveryManager;
pub use self::avahi_discovery_manager::AvahiDiscoveryManager;

pub mod discovery_manager;
mod avahi_discovery_manager;