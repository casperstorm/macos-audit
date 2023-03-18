use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::Command;

use plist::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct EntitlementList(HashMap<Entitlement, bool>);

impl IntoIterator for EntitlementList {
    type Item = (Entitlement, bool);
    type IntoIter = IntoIter<Entitlement, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entitlement(String);

impl Display for Entitlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] plist::Error),
    #[error("invalid data")]
    InvalidDataType,
}

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
    let plist = Value::from_reader(reader)?;

    Ok(EntitlementList(
        plist
            .as_dictionary()
            .ok_or(Error::InvalidDataType)?
            .into_iter()
            .filter_map(|(key, value)| {
                value
                    .as_boolean()
                    .map(|value| (Entitlement(key.clone()), value))
            })
            .collect::<HashMap<Entitlement, bool>>(),
    ))
}
