error_chain! {
    types {
        Error, ErrorKind;
    }

    foreign_links {
        ::protobuf::error::ProtobufError, Protobuf;
        ::uuid::ParseError, UuidParseError;
    }

    errors {
        UnknownMessageType(ty: String) {
            description("unknown message type")
            display("unknown message type: '{}'", ty)
        }
    }
}
