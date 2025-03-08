//! Terminal I/O stream operations.
//!
//! This API automatically supports setting arbitrary I/O speeds, on any
//! platform that supports them, including Linux and the BSDs.
//!
//! The [`speed`] module contains various predefined speed constants which
//! are more likely to be portable, however any `u32` value can be passed to
//! [`Termios::set_input_speed`], and it will simply fail if the speed is not
//! supported by the platform.

#[cfg(any(not(any(target_os = "espidf", target_os = "haiku", target_os = "wasi")), target_vendor = "wasmer"))]
mod ioctl;
#[cfg(any(not(target_os = "wasi"), target_vendor = "wasmer"))]
mod tc;
#[cfg(not(windows))]
mod tty;
#[cfg(any(not(any(target_os = "espidf", target_os = "wasi")), target_vendor = "wasmer"))]
mod types;

#[cfg(any(not(any(target_os = "espidf", target_os = "haiku", target_os = "wasi")), target_vendor = "wasmer"))]
pub use ioctl::*;
#[cfg(any(not(target_os = "wasi"), target_vendor = "wasmer"))]
pub use tc::*;
#[cfg(not(windows))]
pub use tty::*;
#[cfg(any(not(any(target_os = "espidf", target_os = "wasi")), target_vendor = "wasmer"))]
pub use types::*;
