use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::Command;

use entitlement::{Entitlement, EntitlementList, Error, Value};

pub mod entitlement;
pub mod verify;

pub fn entitlements(path: &Path) -> Result<EntitlementList, Error> {
    let output = Command::new("codesign")
        .arg("-d")
        .arg("--entitlements")
        .arg(":-")
        .arg(path)
        .output()
        .expect("codesign command error")
        .stdout;

    let file = Cursor::new(output);
    let reader = BufReader::new(file);
    let plist = plist::Value::from_reader(reader)?;

    Ok(plist
        .as_dictionary()
        .ok_or(Error::InvalidDataType)?
        .into_iter()
        .map(|(key, value)| (Entitlement::unchecked_from(key), value.clone().into()))
        .collect::<HashMap<Entitlement, Value>>()
        .into())
}
