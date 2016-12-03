use Error;
use discovery;
use back::net;

use std::collections::VecDeque;
use std::io::prelude::*;
use std::{mem, io};
use std;

use mio;
use openssl::ssl;

use byteorder::{ByteOrder, BigEndian, WriteBytesExt};

const CAST_PORT: u16 = 8009;

/// The integer type used to prefix packet sizes.
type SizePrefix = u32;
type RawPacket = Vec<u8>;

pub struct Transport
{
    token: mio::Token,
    stream: Stream,
    reader: Reader,

    /// The packets that we have received so far.
    received_packets: VecDeque<RawPacket>,
    /// The packets we need to send.
    queued_packets: VecDeque<RawPacket>,
}

enum Stream
{
    /// HACK TO APPEASE THE BORROW CHECKER.
    None,
    /// Waiting for the connection to be established.
    PendingConnected(mio::tcp::TcpStream),
    Connected(ssl::SslStream<mio::tcp::TcpStream>),
}

#[derive(Debug, PartialEq, Eq)]
enum Reader
{
    /// We are currently reading the size from the stream.
    ReadingSize {
        /// The bytes we have received so far of the size integer.
        bytes: Vec<u8>,
    },
    /// We are currently reading the body from the stream.
    ReadingBody {
        /// The bytes we have received so far.
        bytes: Vec<u8>,
        /// The total size of the body.
        size: usize,
    },
}

impl Transport
{
    pub fn new(stream: mio::tcp::TcpStream,
               io: &mut net::Io) -> Result<Self, Error> {
        let token = io.create_token();

        io.poll.register(&stream, token,
                         mio::Ready::writable() | mio::Ready::readable() | mio::Ready::hup(),
                         mio::PollOpt::level())?;

        Ok(Transport {
            token: token,
            stream: Stream::PendingConnected(stream),
            reader: Reader::new(),
            received_packets: VecDeque::new(),
            queued_packets: VecDeque::new(),
        })
    }

    /// Connect to a Cast device that was discovered/
    pub fn connect_to(device: &discovery::DeviceInfo,
                      io: &mut net::Io) -> Result<Self, Error> {
        let ip_addr = std::net::IpAddr::V4(device.ip_addr);
        let socket_addr = std::net::SocketAddr::new(ip_addr, CAST_PORT);

        let stream = mio::tcp::TcpStream::connect(&socket_addr)?;

        Transport::new(stream, io)
    }

    pub fn send(&mut self, data: Vec<u8>) -> Result<(), Error> {
        self.queued_packets.push_back(data);
        Ok(())
    }

    pub fn receive(&mut self) -> ::std::collections::vec_deque::Drain<Vec<u8>> {
        self.received_packets.drain(..)
    }

    pub fn update(&mut self) {
        if let Stream::Connected(ref mut stream) = self.stream {
            let mut packets = Vec::new();
            self.reader.read(stream, &mut packets).unwrap();
            self.received_packets.extend(packets);
        }
    }

    pub fn handle_event(&mut self, event: mio::Event)
        -> Result<(), Error> {
        if event.token() == self.token {
            if event.kind().is_readable() {
                if let Stream::Connected(ref mut stream) = self.stream {
                    let mut packets = Vec::new();
                    self.reader.read(stream, &mut packets)?;
                    self.received_packets.extend(packets);
                }
            }

            if event.kind().is_writable() {
                let current_stream = mem::replace(&mut self.stream, Stream::None);

                self.stream = match current_stream {
                    Stream::PendingConnected(stream) => {
                        use std::os::unix::io::AsRawFd;
                        use libc;

                        let fd = stream.as_raw_fd();


                        // SslStream doesn't seem to properly support non-blocking sockets.
                        // We temporarily turn off the flag until the handshake finishes.
                        let status = unsafe { libc::fcntl(fd, libc::F_SETFL, libc::fcntl(fd, libc::F_GETFL, 0) & !libc::O_NONBLOCK) };
                        if status == -1 { panic!("could not mark socket as blocking"); }

                        let mut connector_builder = ssl::SslConnectorBuilder::new(ssl::SslMethod::tls()).unwrap();
                        connector_builder.builder_mut().set_verify(ssl::SSL_VERIFY_NONE);

                        let connector = connector_builder.build();
                        let ssl_stream = connector.danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(stream).unwrap();

                        let status = unsafe { libc::fcntl(fd, libc::F_SETFL, libc::fcntl(fd, libc::F_GETFL, 0) | libc::O_NONBLOCK) };
                        if status == -1 { panic!("could not mark socket as non-blocking"); }

                        Stream::Connected(ssl_stream)
                    },
                    Stream::Connected(mut stream) => {
                        if let Some(raw_packet) = self.queued_packets.pop_front() {
                            println!("sending packet");

                            stream.write_u32::<BigEndian>(raw_packet.len() as u32)?;
                            stream.write(&raw_packet)?;
                        }
                        Stream::Connected(stream)
                    },
                    Stream::None => unreachable!(),
                };
            }
        } else {
            unreachable!();
        }
        Ok(())
    }
}

impl Reader
{
    pub fn new() -> Self {
        Reader::ReadingSize { bytes: Vec::new() }
    }

    /// Attempts to read data from a stream into a list of packets.
    pub fn read(&mut self,
                read: &mut Read,
                packets: &mut Vec<RawPacket>) -> Result<(), Error> {
        loop {
            if !self.progress_state(read, packets)? { break };
        }

        Ok(())
    }

    /// Reads data from the stream and attempts to move to
    /// the next state.
    ///
    /// Returns `Ok(true)` if it is possible that we have enough
    /// data to progress to the next state.
    fn progress_state(&mut self,
                      read: &mut Read,
                      packets: &mut Vec<RawPacket>) -> Result<bool, Error> {
        let current_state = mem::replace(self, Reader::new());

        let (can_progress, new_state) = match current_state {
            Reader::ReadingSize { mut bytes } => {
                let bytes_remaining = mem::size_of::<SizePrefix>() - bytes.len();

                let extra_bytes: Result<Vec<_>, _> = read.bytes().take(bytes_remaining).collect();

                match extra_bytes {
                    Ok(extra_bytes) => bytes.extend(extra_bytes),
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // do nothing
                    },
                    Err(e) => return Err(e)?,
                }

                if bytes.len() == mem::size_of::<SizePrefix>() {
                    let body_size = BigEndian::read_u32(&bytes);

                    (true, Reader::ReadingBody {
                        bytes: Vec::new(),
                        size: body_size as usize,
                    })
                } else {
                    (false, Reader::ReadingSize { bytes: bytes })
                }
            },
            Reader::ReadingBody { mut bytes, size } => {
                let bytes_remaining = size - bytes.len();

                let extra_bytes: Result<Vec<_>, _> = read.bytes().take(bytes_remaining).collect();
                bytes.extend(extra_bytes?);

                if bytes.len() == size {
                    // We have finished reading a packet.
                    packets.push(bytes);
                    (true, Reader::new())
                } else {
                    (false, Reader::ReadingBody { bytes: bytes, size: size })
                }
            },
        };

        *self = new_state;

        Ok(can_progress)
    }
}

#[cfg(test)]
mod test
{
    mod reader
    {
        use super::super::{Reader, RawPacket};
        use std::io;

        fn read_data(data: &[u8]) -> (Reader, Vec<RawPacket>) {
            let mut cursor = io::Cursor::new(data);

            let mut reader = Reader::new();
            let mut packets = Vec::new();

            reader.read(&mut cursor, &mut packets).unwrap();
            (reader, packets)
        }

        #[test]
        fn it_reads_size_prefix_at_once() {
            let (reader, packets) = read_data(&[0,0,0,10]);

            assert_eq!(packets.len(), 0);
            assert_eq!(reader, Reader::ReadingBody {
                bytes: Vec::new(), size: 10,
            });
        }

        #[test]
        fn it_reads_since_prefix_in_pieces() {
            let (mut reader, _) = read_data(&[0,0,0]);

            assert_eq!(reader, Reader::ReadingSize { bytes: vec![0,0,0] });

            reader.read(&mut io::Cursor::new([30]), &mut Vec::new()).unwrap();
            assert_eq!(reader, Reader::ReadingBody { size: 30, bytes: vec![] });
        }

        #[test]
        fn it_reads_size_and_body_at_once() {
            let (reader, packets) = read_data("\x00\x00\x00\x05hello".as_bytes());

            assert_eq!(reader, Reader::ReadingSize { bytes: vec![] });
            assert_eq!(packets, vec!["hello".as_bytes()]);
        }

        #[test]
        fn it_reads_body_in_pieces() {
            let (mut reader, mut packets) = read_data("\x00\x00\x00\x05wo".as_bytes());

            assert_eq!(reader, Reader::ReadingBody { size: 5, bytes: "wo".as_bytes().to_owned() });
            assert_eq!(packets.len(), 0);

            reader.read(&mut io::Cursor::new("rld"), &mut packets).unwrap();
            assert_eq!(reader, Reader::ReadingSize { bytes: Vec::new() });

            assert_eq!(packets, vec!["world".as_bytes()]);
        }
    }
}
