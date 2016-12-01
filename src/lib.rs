#![recursion_limit = "1024"]

pub use self::errors::{Error, ErrorKind};

pub mod discovery;
pub mod net;
pub mod errors;

extern crate mdns;
extern crate mio;
extern crate byteorder;
extern crate uuid;
#[macro_use]
extern crate error_chain;
