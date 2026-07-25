use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Eq, Hash, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Path,
    Post,
    Put,
    Trace
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}