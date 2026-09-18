use std::fmt::{Display, Formatter};
use serde::Deserialize;

/// The method being used in an HTTP request.
#[derive(Eq, Hash, PartialEq, Clone, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    /// The HTTP request [`Method`] for deleting a resource.
    Delete,

    /// The HTTP request [`Method`] for retrieving a resource.
    Get,

    /// The HTTP request [`Method`] for retrieving header information for a resource.
    Head,

    /// The HTTP request [`Method`] for updating a resource.
    Patch,

    /// The HTTP request [`Method`] for creating a resource.
    Post,

    /// The HTTP request [`Method`] for overwriting a resource.
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