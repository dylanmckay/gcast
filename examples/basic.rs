extern crate gcast;

fn main() {
    gcast::discovery::run(|device| {
        println!("device: {:#?}", device);
    }).unwrap();
}
