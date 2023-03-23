use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] plist::Error),
    #[error("invalid data")]
    InvalidDataType,
}

#[derive(Debug, Clone)]
pub struct Value(plist::Value);

impl From<plist::Value> for Value {
    fn from(value: plist::Value) -> Self {
        Value(value)
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
pub struct EntitlementList(HashMap<Entitlement, Value>);

impl From<HashMap<Entitlement, Value>> for EntitlementList {
    fn from(value: HashMap<Entitlement, Value>) -> Self {
        Self(value)
    }
}

impl IntoIterator for EntitlementList {
    type Item = (Entitlement, Value);
    type IntoIter = IntoIter<Entitlement, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
