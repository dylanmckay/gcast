extern crate gcast;

use gcast::back;

fn main() {
    gcast::discovery::run(|device_info| {
        println!("discovered device: {:#?}", device_info);

        let mut connection = gcast::back::Connection::connect_to(&device_info).unwrap();

        let mut message = back::protocol::CastMessage::new();

        message.set_protocol_version(back::protocol::CastMessage_ProtocolVersion::CASTV2_1_0);
        message.set_source_id("sender-0".to_owned());
        message.set_destination_id("receiver-0".to_owned());
        message.set_namespace("urn:x-cast:com.google.cast.tp.connection".to_owned());
        message.set_payload_type(back::protocol::CastMessage_PayloadType::STRING);
        message.set_payload_utf8("{ \"type\": \"CONNECT\" }".to_owned());

        connection.send(&message).unwrap();
    }).unwrap();
}
