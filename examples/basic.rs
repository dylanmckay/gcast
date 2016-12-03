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
    {
        let mut message = back::protocol::CastMessage::new();

        message.set_protocol_version(back::protocol::CastMessage_ProtocolVersion::CASTV2_1_0);
        message.set_source_id("sender-0".to_owned());
        message.set_destination_id("receiver-0".to_owned());
        message.set_namespace("urn:x-cast:com.google.cast.tp.connection".to_owned());
        message.set_payload_type(back::protocol::CastMessage_PayloadType::STRING);
        message.set_payload_utf8("{ \"type\": \"CONNECT\" }".to_owned());

        connection.send(&message).unwrap();
    }

    {
        let mut message = back::protocol::CastMessage::new();

        message.set_protocol_version(back::protocol::CastMessage_ProtocolVersion::CASTV2_1_0);
        message.set_source_id("sender-0".to_owned());
        message.set_destination_id("receiver-0".to_owned());
        message.set_namespace("urn:x-cast:com.google.cast.receiver".to_owned());
        message.set_payload_type(back::protocol::CastMessage_PayloadType::STRING);
        message.set_payload_utf8("{ \"type\": \"GET_STATUS\" }".to_owned());

        connection.send(&message).unwrap();
    }

    'poll_loop: loop {
        io.poll.poll(&mut io.events, None).unwrap();

        for event in io.events.iter()  {
            if event.kind().is_hup() {
                break 'poll_loop;
            }

            connection.handle_event(event).unwrap();

            for packet in connection.receive().unwrap() {
                println!("packet: {:#?}", packet);
            }
        }
    }

    println!("Cast device disconnected");
}
