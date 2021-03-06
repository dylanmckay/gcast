//! The core `Device` type.

use {DeviceInfo, ApplicationId, SessionId, Status, Event, Error,
     VolumeLevel};
use back;

use std::collections::VecDeque;
use std;

use mio;

/// If the internal event queue gets too big, truncate the oldest events.
const EVENT_QUEUE_MAXIMUM_COUNT: usize = 500;

/// The string we will use to identify ourselves in messages.
const SENDER_ID: &'static str = "sender-0";
/// The string we will use to identify the Cast device in messages.
const RECEIVER_ID: &'static str = "sender-0";

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
    /// A queue that holds the events that have occurred on this device.
    event_queue: VecDeque<Event>,
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
            event_queue: VecDeque::new(),
        }
    }

    /// Connect to a receiver.
    pub fn connect(info: DeviceInfo, io: &mut back::net::Io)
        -> Result<Self, Error> {
        let connection = back::Connection::connect_to(&info, io)?;
        let mut device = Device::new(info, connection);

        // Establish a virtual connection
        device.send_message(back::protocol::namespace::connection(),
                            back::protocol::MessageKind::Connect)?;

        Ok(device)
    }

    /// Asks the Chromecast for its current status.
    pub fn update_status(&mut self) -> Result<(), Error> {
        self.send_message(back::protocol::namespace::receiver(),
                          back::protocol::MessageKind::GetStatus)
    }

    /// Launch an application.
    pub fn launch(&mut self, app_id: ApplicationId) -> Result<(), Error> {
        self.send_message(back::protocol::namespace::receiver(),
            back::protocol::MessageKind::Launch {
                app_id: app_id,
                request_id: 1,
        })
    }

    /// Stop a running application.
    ///
    /// Arguments:
    ///
    /// * `session_id` - The identifer of the session.
    pub fn stop(&mut self, session_id: SessionId) -> Result<(), Error> {
        self.send_message(back::protocol::namespace::receiver(),
            back::protocol::MessageKind::Stop {
                session_id: session_id,
        })
    }

    /// Sets the volume of the Cast device.
    /// **NOTE**: This API is likely to change.
    pub fn set_volume(&mut self, level: Option<VolumeLevel>, muted: Option<bool>)
        -> Result<(), Error> {
        self.send_message(back::protocol::namespace::receiver(),
            back::protocol::MessageKind::SetVolume {
                level: level,
                muted: muted,
        })
    }

    /// Handle an IO event.
    pub fn handle_io(&mut self, event: mio::Event) -> Result<(), Error> {
        self.connection.handle_event(event)?;
        self.process_incoming()
    }

    /// Consumes all of the events that have occurred on this device.
    pub fn events(&mut self) -> VecDeque<Event> {
        std::mem::replace(&mut self.event_queue, VecDeque::new())
    }

    /// Gets information about the Cast device.
    pub fn info(&self) -> &DeviceInfo { &self.info }

    /// Get the current status of the receiver.
    pub fn status(&self) -> Option<&Status> { self.status.as_ref() }

    /// Sends a message.
    fn send_message(&mut self,
                    namespace: back::protocol::Namespace,
                    kind: back::protocol::MessageKind) -> Result<(), Error> {
        self.connection.send(&back::protocol::Message {
            source: back::protocol::EndpointName(SENDER_ID.to_owned()),
            destination: back::protocol::EndpointName(RECEIVER_ID.to_owned()),
            namespace: namespace,
            kind: kind,
        })
    }

    /// Process all incoming messages.
    fn process_incoming(&mut self) -> Result<(), Error> {
        for message in self.connection.receive()? {
            match message.kind {
                back::protocol::MessageKind::Ping => {
                    self.connection.send(&back::protocol::Message {
                        source: message.destination.clone(),
                        destination: message.source.clone(),
                        namespace: message.namespace.clone(),
                        kind: back::protocol::MessageKind::Pong,
                    }).expect("failed to send PONG");
                },
                back::protocol::MessageKind::ReceiverStatus(status) => {
                    self.status = Some(status);
                    self.add_event(Event::StatusUpdated);
                },
                msg => {
                    println!("received unimplemented message: {:?}", msg);
                },
            }
        }

        Ok(())
    }

    fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);

        if self.event_queue.len() > EVENT_QUEUE_MAXIMUM_COUNT {
            self.event_queue.drain(EVENT_QUEUE_MAXIMUM_COUNT-1..);
        }
    }
}
