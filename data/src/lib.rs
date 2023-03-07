use plist::Plist;
use std::{
    io::{BufRead, BufReader, Cursor},
    path::Path,
    process::Command,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Entitlement {
    #[serde(rename = "com.apple.security.device.camera")]
    key: String,
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

    let file = Cursor::new(output);
    let mut reader = BufReader::new(file);
    let plist = Plist::from_reader(&mut reader).unwrap();

    println!("{:?}", plist);
}
