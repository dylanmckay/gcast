#![recursion_limit = "1024"]

pub extern crate gcast_wire as wire;

pub use self::message::{Message, MessageKind, Namespace, EndpointName};
pub use self::status::{Status, Volume, VolumeLevel};
pub use self::errors::{Error, ErrorKind};

pub mod message;
pub mod status;
pub mod errors;

extern crate protobuf;
extern crate uuid;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate json;

/// Namespace definition constants.
pub mod namespace {
    use super::Namespace;

    /// Gets the 'connection' namespace.
    pub fn connection() -> Namespace {
        Namespace("urn:x-cast:com.google.cast.tp.connection".to_owned())
    }

    /// Gets the 'heartbeat' namespace.
    ///
    /// This is used for sending `PING` and `PONG`.
    pub fn heartbeat() -> Namespace {
        Namespace("urn:x-cast:com.google.cast.tp.heartbeat".to_owned())
    }

    /// Gets the 'receiver' namespace.
    pub fn receiver() -> Namespace {
        Namespace("urn:x-cast:com.google.cast.receiver".to_owned())
    }

    /// Gets the 'deviceauth' namespace.
    pub fn device_auth() -> Namespace {
        Namespace("cast:com.google.cast.tp.deviceauth".to_owned())
    }
}
