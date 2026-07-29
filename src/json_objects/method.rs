use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Eq, Hash, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Delete,
    Get,
    Head,
    Patch,
    Post,
    Put,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Into<String> for Method {
    fn into(self) -> String {
        match self {
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Patch => "PATCH",
            Method::Post => "POST",
            Method::Put => "PUT",
        }.to_string()
    }
}