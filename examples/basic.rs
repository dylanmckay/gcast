extern crate gcast;

use gcast::back;

use std::time::Duration;

fn main() {
    let poll_duration = Duration::from_secs(1);
    let mut cast_device_infos = Vec::new();

    gcast::discovery::run(poll_duration, |device_info| {
        println!("discovered device: {:#?}", device_info);
        cast_device_infos.push(device_info);
    }).unwrap();

    for device_info in cast_device_infos {
        let mut io = gcast::back::net::Io::new().unwrap();
        let mut connection = gcast::back::Connection::connect_to(&device_info, &mut io).unwrap();

        let mut message = back::protocol::CastMessage::new();

        message.set_protocol_version(back::protocol::CastMessage_ProtocolVersion::CASTV2_1_0);
        message.set_source_id("sender-0".to_owned());
        message.set_destination_id("receiver-0".to_owned());
        message.set_namespace("urn:x-cast:com.google.cast.tp.connection".to_owned());
        message.set_payload_type(back::protocol::CastMessage_PayloadType::STRING);
        message.set_payload_utf8("{ \"type\": \"CONNECT\" }".to_owned());

        connection.send(&message).unwrap();

        io.poll.poll(&mut io.events, None).unwrap();

        loop {
            for event in io.events.iter() {
                connection.handle_event(event).unwrap();

                for packet in connection.receive() {
                    println!("packet: {:#?}", packet);
                }
            }
        }
    }
}
