use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::Command;

use plist::Plist;
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
    #[error("invalid data: {0}")]
    InvalidData(String),
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
    let mut reader = BufReader::new(file);
    let plist = Plist::from_reader(&mut reader).map_err(Error::from)?;

    if let Plist::Dict(dict) = plist {
        Ok(EntitlementList(
            dict.into_iter()
                .filter_map(|(key, value)| {
                    if let Plist::Boolean(value) = value {
                        Some((Entitlement(key), value))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<Entitlement, bool>>(),
        ))
    } else {
        Err(Error::InvalidData(format!("{:?}", plist)))
    }
}
