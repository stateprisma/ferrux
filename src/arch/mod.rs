#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
pub mod amd64;
#[cfg(target_arch = "x86_64")]
pub use amd64::*;
