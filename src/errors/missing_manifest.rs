use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct MissingManifest;

impl Display for MissingManifest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Manifest could not be found.")
    }
}

impl Error for MissingManifest {}