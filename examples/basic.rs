extern crate gcast;

fn main() {
    gcast::discovery::run(|device_info| {
        println!("discovered device: {:#?}", device_info);
    }).unwrap();
}
