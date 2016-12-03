extern crate gcast;

use gcast::back;

use std::time::Duration;

#[allow(dead_code)]
fn discover() -> Vec<gcast::DeviceInfo> {
    let poll_duration = Duration::from_secs(3);
    let mut cast_device_infos = Vec::new();

    gcast::discovery::run(poll_duration, |device_info| {
        println!("discovered device: {:#?}", device_info);
        cast_device_infos.push(device_info);
    }).unwrap();

    cast_device_infos
}

fn main() {
    let device_info = gcast::DeviceInfo {
        ip_addr: "192.168.1.102".parse().unwrap(),
        uuid: "d7288042-190b-5974-aa3b-2558f1cb0c0e".parse().unwrap(),
    };

    let mut io = gcast::back::net::Io::new().unwrap();
    let mut connection = gcast::back::Connection::connect_to(&device_info, &mut io).unwrap();

    // Establish a virtual connection
    connection.send(&back::protocol::Message {
        source: back::protocol::EndpointName("sender-0".to_owned()),
        destination: back::protocol::EndpointName("receiver-0".to_owned()),
        namespace: back::protocol::namespace::connection(),
        kind: back::protocol::MessageKind::Connect,
    }).expect("failed to send CONNECT");

    // Get the current status of the Cast device.
    connection.send(&back::protocol::Message {
        source: back::protocol::EndpointName("sender-0".to_owned()),
        destination: back::protocol::EndpointName("receiver-0".to_owned()),
        namespace: back::protocol::namespace::receiver(),
        kind: back::protocol::MessageKind::GetStatus,
    }).expect("failed to send CONNECT");

    /// Launch the YouTube app.
    connection.send(&back::protocol::Message {
        source: back::protocol::EndpointName("sender-0".to_owned()),
        destination: back::protocol::EndpointName("receiver-0".to_owned()),
        namespace: back::protocol::namespace::receiver(),
        kind: back::protocol::MessageKind::Launch {
            app_id: "YouTube".to_owned(),
            request_id: 1,
        },
    }).expect("failed to send LAUNCH");

    'poll_loop: loop {
        io.poll.poll(&mut io.events, Some(Duration::from_millis(200))).unwrap();

        for event in io.events.iter()  {
            if event.kind().is_hup() {
                break 'poll_loop;
            }

            connection.handle_event(event).unwrap();
        }

        for message in connection.receive().unwrap() {
            match message.kind {
                back::protocol::MessageKind::Ping => {
                    println!("received PING, responding with PONG: {:#?}", message);

                    connection.send(&back::protocol::Message {
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
    }

    println!("Cast device disconnected");
}
