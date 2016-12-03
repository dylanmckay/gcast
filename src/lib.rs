#![recursion_limit = "1024"]

pub use self::errors::{Error, ErrorKind};
pub use self::discovery::DeviceInfo;
pub use self::device::Device;
pub use self::back::protocol::{Status, Volume, VolumeLevel};
pub use self::event::Event;

pub mod discovery;
pub mod errors;
pub mod back;
pub mod device;
pub mod event;

extern crate mdns;
extern crate mio;
extern crate byteorder;
extern crate uuid;
#[macro_use]
extern crate error_chain;
extern crate openssl;
extern crate libc;
