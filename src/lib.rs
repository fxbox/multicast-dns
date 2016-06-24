#[macro_use]
extern crate log;

extern crate libc;

mod adapters;
#[cfg(target_os = "linux")]
mod bindings;

pub mod discovery;
pub mod host;
pub use adapters::errors;