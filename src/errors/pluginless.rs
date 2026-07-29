use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pluginless;

impl Display for Pluginless {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plugin could not be found.")
    }
}

impl Error for Pluginless {}