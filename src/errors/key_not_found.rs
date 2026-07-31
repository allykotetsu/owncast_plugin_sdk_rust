use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct KeyNotFound(pub String);

impl Display for KeyNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let KeyNotFound(key) = self;
        write!(f, "Key {key} could not be found.")
    }
}

impl Error for KeyNotFound {}