use Error;

use discovery;

use std::io::prelude::*;
use std::mem;
use std;

use mio;

use byteorder::{ByteOrder, BigEndian};

const CAST_PORT: u16 = 8009;

/// The integer type used to prefix packet sizes.
type SizePrefix = u32;
type RawPacket = Vec<u8>;

pub struct Connection
{
    stream: mio::tcp::TcpStream,
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

impl Connection
{
    pub fn new(stream: mio::tcp::TcpStream) -> Self {
        Connection { stream: stream }
    }

    /// Connect to a Cast device that was discovered/
    pub fn connect_to(device: &discovery::DeviceInfo) -> Result<Self, Error> {
        let ip_addr = std::net::IpAddr::V4(device.ip_addr);
        let socket_addr = std::net::SocketAddr::new(ip_addr, CAST_PORT);

        let stream = mio::tcp::TcpStream::connect(&socket_addr)?;

        Ok(Connection::new(stream))
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
                bytes.extend(extra_bytes?);

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
