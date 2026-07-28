use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Dbg(pub String);

impl Display for Dbg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Dbg(msg) = self;
        write!(f, "{msg}")
    }
}

impl Error for Dbg {}