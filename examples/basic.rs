extern crate gcast;

use std::time::Duration;

fn main() {
    let device_info = gcast::DeviceInfo {
        ip_addr: "192.168.1.102".parse().unwrap(),
        uuid: "d7288042-190b-5974-aa3b-2558f1cb0c0e".parse().unwrap(),
    };

    let mut io = gcast::back::net::Io::new().unwrap();
    let mut device = gcast::Device::connect(device_info, &mut io).unwrap();

    /// Launch the YouTube app.
    device.launch(gcast::apps::youtube()).unwrap();

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
