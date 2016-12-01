use std::process::Command;

fn main() {
    let protoc_status = Command::new("protoc")
                                .arg("src/back/protocol/cast_channel.proto")
                                .args(&["--rust_out", "src/back/protocol"])
                                .status()
                                .expect("failed to run protoc");
    assert!(protoc_status.success());
}
