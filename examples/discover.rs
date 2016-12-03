extern crate gcast;

use std::time::Duration;

/// Finds all Cast devices on the network and launches the YouTube app.
fn main() {
    let poll_duration = Duration::from_secs(10);

    let mut io = gcast::back::net::Io::new().unwrap();

    gcast::discovery::run(poll_duration, |device_info| {
        println!("found cast device on: {:#?} with UUID {}, launching YouTube",
                 device_info.ip_addr, device_info.uuid);

        let mut device = gcast::Device::connect(device_info, &mut io).unwrap();
        device.launch(gcast::apps::youtube()).unwrap();
    }).unwrap();
}
