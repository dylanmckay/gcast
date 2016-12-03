//! A handy crate for interacting with Google Cast devices.
//!
//! Features
//!
//! * Cast device discovery on local network
//! * Implementation of the CASTV2 protocol
//! * Query device status
//! * Launch applications
//!
//! # Discovery
//!
//! Discovery-related functionality lives inside the `discovery` module.
//!
//! # Communication
//!
//! The most important type is the `Device` struct. It maintains a network
//! connection to the Cast device and it abstracts over the protocol, making it
//! easy to do things.
//!
//! # Applications
//!
//! A list of valid application identifers can be found inside the `apps` module.
//!
//! You can launch and manage apps using these identifiers.

#![recursion_limit = "1024"]

pub use self::errors::{Error, ErrorKind};
pub use self::discovery::DeviceInfo;
pub use self::device::Device;
pub use self::back::protocol::{ApplicationId, Status, Volume, VolumeLevel};
pub use self::event::Event;

pub mod discovery;
pub mod errors;
pub mod back;
pub mod device;
pub mod event;
pub mod apps;

extern crate mdns;
extern crate mio;
extern crate byteorder;
extern crate uuid;
#[macro_use]
extern crate error_chain;
extern crate openssl;
extern crate libc;
