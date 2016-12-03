//! All backend related networking/protocol code.
//!
//! This module consists of two parts - `net` and `protocol`.
//!
//! The networking code and the protocol code are completely independent,
//! and are located in separate submodules.
//!
//! The two are brought together into the `Connection` struct.

pub use self::connection::Connection;

pub mod connection;

pub mod net;

pub extern crate gcast_protocol as protocol;
