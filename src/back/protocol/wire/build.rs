use std::process::Command;

fn main() {
    let protoc_status = Command::new("protoc")
                                .arg("cast_channel.proto")
                                .args(&["--rust_out", "."])
                                .status()
                                .expect("failed to run protoc");
    assert!(protoc_status.success());
}
