use std::{path::Path, process::Command};

struct Entitlement {
    key: String,
    value: bool,
}

pub fn entitlements(path: &Path) {
    let output = Command::new("codesign")
        .arg("-d")
        .arg("--entitlements")
        .arg(":-")
        .arg(path)
        .output()
        .expect("codesign command error")
        .stdout;
    let payload = String::from_utf8(output).unwrap();

    println!("{:?}", payload);
}
