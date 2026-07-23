use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Eq, Hash, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATH,
    POST,
    PUT,
    TRACE
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}