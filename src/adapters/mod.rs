pub use self::adapter::Adapter;

#[cfg(target_os = "linux")]
mod avahi;
#[cfg(target_os = "linux")]
pub use adapters::avahi::AvahiAdapter as PlatformDependentAdapter;

#[cfg(not(target_os = "linux"))]
mod fake;
#[cfg(not(target_os = "linux"))]
pub use adapters::fake::FakeAdapter as PlatformDependentAdapter;

pub mod adapter;
pub mod errors;