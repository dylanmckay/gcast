use Error;
use discovery;
use back::{net, protocol};

use protobuf::Message;
use protobuf;
use mio;

/// A connection to a Cast device.
pub struct Connection
{
    pub transport: net::Transport,
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
    pub fn send(&mut self, message: &protocol::wire::CastMessage) -> Result<(), Error> {
        let bytes = message.write_to_bytes()?;
        self.transport.send(bytes)?;
        Ok(())
    }

    /// Consumes all packets that have been received.
    pub fn receive(&mut self) -> Result<::std::vec::IntoIter<protocol::wire::CastMessage>, Error> {
        let result: Result<Vec<protocol::wire::CastMessage>, _> = self.transport.receive().map(|raw_packet| {
            protobuf::parse_from_bytes(&raw_packet)
        }).collect();

        Ok(result?.into_iter())
    }

    /// Handles an IO event.
    pub fn handle_event(&mut self, event: mio::Event) -> Result<(), Error> {
        self.transport.handle_event(event)
    }
}
