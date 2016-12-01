use Error;
use discovery;
use back::{net, protocol};

use protobuf::Message;
use mio;

/// A connection to a Cast device.
pub struct Connection
{
    transport: net::Transport,
}

impl Connection
{
    pub fn connect_to(device: &discovery::DeviceInfo,
                      io: &mut net::Io) -> Result<Self, Error> {
        Ok(Connection {
            transport: net::Transport::connect_to(device, io)?,
        })
    }

    /// Sends a packet through the connection.
    pub fn send(&mut self, message: &protocol::CastMessage) -> Result<(), Error> {
        let bytes = message.write_to_bytes()?;
        self.transport.send(bytes)?;
        Ok(())
    }

    /// Consumes all packets that have been received.
    pub fn receive(&mut self) -> ::std::collections::vec_deque::Drain<Vec<u8>> {
        self.transport.receive()
    }

    /// Handles an IO event.
    pub fn handle_event(&mut self, event: mio::Event) -> Result<(), Error> {
        self.transport.handle_event(event)
    }
}
