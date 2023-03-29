use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{BufReader, Cursor};
use std::path::Path;
use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] plist::Error),
    #[error("invalid data")]
    InvalidDataType,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Value(plist::Value);

impl From<plist::Value> for Value {
    fn from(value: plist::Value) -> Self {
        Value(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entitlement(String);

impl Entitlement {
    pub(crate) fn unchecked_from(value: impl ToString) -> Self {
        Entitlement(value.to_string())
    }
}

impl Display for Entitlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct List(HashMap<Entitlement, Value>);

impl From<HashMap<Entitlement, Value>> for List {
    fn from(value: HashMap<Entitlement, Value>) -> Self {
        Self(value)
    }
}

impl IntoIterator for List {
    type Item = (Entitlement, Value);
    type IntoIter = IntoIter<Entitlement, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<&Path> for List {
    type Error = Error;

    fn try_from(path: &Path) -> Result<List, Error> {
        let output = Command::new("codesign")
            .arg("-d")
            .arg("--entitlements")
            .arg(":-")
            .arg(path)
            .output()?
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
}
