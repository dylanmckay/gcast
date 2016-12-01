use Error;
use discovery;
use back::{net, protocol};

use protobuf::Message;

pub struct Connection
{
    transport: net::Transport,
}

impl Connection
{
    pub fn connect_to(device: &discovery::DeviceInfo) -> Result<Self, Error> {
        Ok(Connection {
            transport: net::Transport::connect_to(device)?,
        })
    }

    pub fn send(&mut self, message: &protocol::CastMessage) -> Result<(), Error> {
        let bytes = message.write_to_bytes()?;
        self.transport.write(&bytes)?;
        Ok(())
    }
}
