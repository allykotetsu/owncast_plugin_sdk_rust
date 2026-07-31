use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Duplicate(pub(crate) String);

impl Display for Duplicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Duplicate(key) = self;
        write!(f, "Duplicate entry for {key}.")
    }
}

impl Error for Duplicate {}