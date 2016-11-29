error_chain! {
    types {
        Error, ErrorKind;
    }

    foreign_links {
        ::mdns::Error, Dns;
    }
}
