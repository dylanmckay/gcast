extern crate gcast;

use std::time::Duration;

/// Connect to a Cast device on a given IP.
/// Launch YouTube and receive status updates.
///
/// Usage:
///
/// cargo run --example basic 192.168.1.102
fn main() {
    let mut io = gcast::back::net::Io::new().unwrap();

    let device_info = self::device_info();
    let mut device = gcast::Device::connect(device_info, &mut io).unwrap();

    /// Launch the YouTube app.
    device.launch(gcast::apps::youtube()).unwrap();
    device.set_volume(None, Some(true)).unwrap();

    'poll_loop: loop {
        io.poll.poll(&mut io.events, Some(Duration::from_millis(200))).unwrap();

        for io_event in io.events.iter()  {
            if io_event.kind().is_hup() {
                break 'poll_loop;
            }

            device.handle_io(io_event).unwrap();
        }

        for event in device.events() {
            match event {
                gcast::Event::StatusUpdated => {
                    println!("device status updated: {:?}", device.status());
                },
            }
        }
    }

    println!("Cast device disconnected");
}

fn device_info() -> gcast::DeviceInfo {
    let ip_addr = match std::env::args().nth(1) {
        Some(ip_addr) => match ip_addr.parse() {
            Ok(ip_addr) => ip_addr,
            Err(e) => {
                println!("invalid IP address: {}", e);
                std::process::exit(1);
            },
        },
        None => {
            println!("no IP address given");
            println!("usage: basic 192.168.1.102");;;;
            std::process::exit(1);
        },
    };

    gcast::DeviceInfo {
        ip_addr: ip_addr,
        uuid: "d7288042-190b-5974-aa3b-2558f1cb0c0e".parse().unwrap(),
    }
}
