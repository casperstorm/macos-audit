use std::fmt::Display;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::entitlement;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File is not a macOS application")]
    InvalidFile,
    #[error(transparent)]
    Entitlement(#[from] entitlement::Error),
}

#[derive(Debug, Clone)]
pub struct Application {
    pub path: PathBuf,
    pub entitlements: entitlement::List,
}

impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

impl TryFrom<&Path> for Application {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Application, Error> {
        if path.extension().map(|ext| ext != "app").unwrap_or(false) {
            return Err(Error::InvalidFile);
        }

        Ok(Application {
            path: path.to_path_buf(),
            entitlements: entitlement::List::try_from(path)?,
        })
    }
}

impl Application {}
