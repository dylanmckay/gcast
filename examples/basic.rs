extern crate cast;

fn main() {
    cast::discovery::run(|device| {
        println!("device: {:#?}", device);
    }).unwrap();
}
