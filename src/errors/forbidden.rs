use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Forbidden;

impl Display for Forbidden {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forbidden.")
    }
}

impl Error for Forbidden {}