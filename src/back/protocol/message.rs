use wire;

/// The version of the CAST protocol we are using.
const PROTOCOL_VERSION: wire::CastMessage_ProtocolVersion = wire::CastMessage_ProtocolVersion::CASTV2_1_0;

/// The namespace a message is send over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Namespace(pub String);

/// A sender/receive ID.
/// Examples:
/// * `receiver-0`
/// * `sender-0`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointName(pub String);

/// A CASTV2 message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message
{
    /// The sender ID of the message.
    pub source: EndpointName,
    pub destination: EndpointName,
    pub namespace: Namespace,
    pub kind: MessageKind,
}

/// A message variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageKind
{
    Connect,
    Ping,
    GetStatus,
}

impl Message
{
    pub fn as_wire_message(&self) -> wire::CastMessage {
        let mut message = wire::CastMessage::new();

        message.set_protocol_version(PROTOCOL_VERSION);
        message.set_source_id(self.source.0.clone());
        message.set_destination_id(self.destination.0.clone());
        message.set_namespace(self.namespace.0.clone());

        match self.kind {
            MessageKind::Ping => {
                message.set_payload_type(wire::CastMessage_PayloadType::STRING);
                message.set_payload_utf8("{\"type\":\"PING\"}".to_owned());
            },
            MessageKind::Connect => {
                message.set_payload_type(wire::CastMessage_PayloadType::STRING);
                message.set_payload_utf8("{ \"type\": \"CONNECT\" }".to_owned());
            },
            MessageKind::GetStatus => {
                message.set_payload_type(wire::CastMessage_PayloadType::STRING);
                message.set_payload_utf8("{ \"type\": \"GET_STATUS\" }".to_owned());
            },
        }

        message
    }
}
