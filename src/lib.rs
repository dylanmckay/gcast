#![recursion_limit = "1024"]

pub use self::errors::{Error, ErrorKind};

pub mod discovery;
pub mod errors;
pub mod back;

extern crate mdns;
extern crate mio;
extern crate byteorder;
extern crate uuid;
extern crate protobuf;
#[macro_use]
extern crate error_chain;
