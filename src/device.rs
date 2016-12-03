use {DeviceInfo, Status, Error};
use back;

use mio;

/// A Cast device.
pub struct Device
{
    /// Information about how to connect to the device.
    info: DeviceInfo,
    /// The current status of the receiver.
    /// This will be set and updated upon receiving a
    /// `RECEIVER_STATUS` message.
    status: Option<Status>,
    /// The network connection.
    connection: back::Connection,
}

impl Device
{
    /// Create a new device instance.
    pub fn new(info: DeviceInfo,
               connection: back::Connection) -> Self {
        Device {
            info: info,
            connection: connection,
            status: None,
        }
    }

    /// Connect to a receiver.
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

    /// Launch the YouTube app.
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

    /// Handle an IO event.
    pub fn handle_event(&mut self, event: mio::Event) -> Result<(), Error> {
        self.connection.handle_event(event)?;
        self.process_incoming()
    }

    /// Gets information about the Cast device.
    pub fn info(&self) -> &DeviceInfo { &self.info }

    /// Get the current status of the receiver.
    pub fn status(&self) -> Option<&Status> { self.status.as_ref() }

    /// Process all incoming messages.
    fn process_incoming(&mut self) -> Result<(), Error> {
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
