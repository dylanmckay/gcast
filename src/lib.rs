#![recursion_limit = "1024"]

pub use self::errors::{Error, ErrorKind};

pub mod discovery;
pub mod errors;

extern crate mdns;
extern crate uuid;
#[macro_use]
extern crate error_chain;
