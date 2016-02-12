pub use self::service_discovery_manager::ServiceDiscoveryManager;
pub use self::avahi_service_discovery_manager::AvahiServiceDiscoveryManager;

pub mod service_discovery_manager;
mod avahi_service_discovery_manager;
mod avah_wrapper;