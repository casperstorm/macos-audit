use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::Command;

use plist::Plist;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct EntitlementList(HashMap<Entitlement, bool>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entitlement(String);

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
