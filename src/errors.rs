error_chain! {
    types {
        Error, ErrorKind;
    }

    foreign_links {
        ::mdns::Error, Dns;
        ::std::io::Error, Io;
        ::protobuf::error::ProtobufError, Protobuf;
        ::openssl::ssl::HandshakeError<::mio::tcp::TcpStream>, SslHandshakeError;
    }
}
