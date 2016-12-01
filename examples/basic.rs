extern crate gcast;

fn main() {
    gcast::discovery::run(|device_info| {
        println!("discovered device: {:#?}", device_info);

        let connection = gcast::back::net::Connection::connect_to(&device_info).unwrap();
    }).unwrap();
}
