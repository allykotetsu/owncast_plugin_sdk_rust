use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct MemNotFound;

impl Display for MemNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory could not be found.")
    }
}

impl Error for MemNotFound {}