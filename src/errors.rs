error_chain! {
    types {
        Error, ErrorKind;
    }

    foreign_links {
        ::mdns::Error, Dns;
        ::std::io::Error, Io;
        ::back::protocol::Error, Protocol;
    }
}
