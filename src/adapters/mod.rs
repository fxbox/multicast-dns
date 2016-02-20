pub use self::adapter::Adapter;

#[cfg(target_os = "linux")]
pub mod avahi;
pub mod fake;
pub mod adapter;