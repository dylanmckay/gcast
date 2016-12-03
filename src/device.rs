use {DeviceInfo, Error};
use back;

use mio;

/// A Cast device.
pub struct Device
{
    info: DeviceInfo,
    connection: back::Connection,
}

impl Device
{
    pub fn new(info: DeviceInfo,
               connection: back::Connection) -> Self {
        Device {
            info: info,
            connection: connection,
        }
    }

    pub fn connect(info: DeviceInfo, io: &mut back::net::Io)
        -> Result<Self, Error> {
        let mut connection = back::Connection::connect_to(&info, io)?;

        // Establish a virtual connection
        connection.send(&back::protocol::Message {
            source: back::protocol::EndpointName("sender-0".to_owned()),
            destination: back::protocol::EndpointName("receiver-0".to_owned()),
            namespace: back::protocol::namespace::connection(),
            kind: back::protocol::MessageKind::Connect,
        }).expect("failed to send CONNECT");

        Ok(Device::new(info, connection))
    }

    pub fn launch_youtube(&mut self) -> Result<(), Error> {
        self.connection.send(&back::protocol::Message {
            source: back::protocol::EndpointName("sender-0".to_owned()),
            destination: back::protocol::EndpointName("receiver-0".to_owned()),
            namespace: back::protocol::namespace::receiver(),
            kind: back::protocol::MessageKind::Launch {
                app_id: "YouTube".to_owned(),
                request_id: 1,
            },
        })

    }

    pub fn handle_event(&mut self, event: mio::Event) -> Result<(), Error> {
        self.connection.handle_event(event)?;
        self.tick()
    }

    /// Gets information about the Cast device.
    pub fn info(&self) -> &DeviceInfo { &self.info }

    fn tick(&mut self) -> Result<(), Error> {
        for message in self.connection.receive()? {
            match message.kind {
                back::protocol::MessageKind::Ping => {
                    println!("received PING, responding with PONG: {:#?}", message);

                    self.connection.send(&back::protocol::Message {
                        source: message.destination.clone(),
                        destination: message.source.clone(),
                        namespace: message.namespace.clone(),
                        kind: back::protocol::MessageKind::Pong,
                    }).expect("failed to send PONG");
                },
                back::protocol::MessageKind::ReceiverStatus { status }=> {
                    println!("receiver status: {}", status);
                },
                msg => {
                    println!("received message: {:?}", msg);
                },
            }
        }

        Ok(())
    }
}
