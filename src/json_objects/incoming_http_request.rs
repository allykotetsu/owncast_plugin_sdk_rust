use extism_pdk::{FromBytes, Json};
use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::method::Method;
use crate::json_objects::user::User;

/// An object representing an incoming HTTP request.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct IncomingHttpRequest {
    /// The HTTP [`Method`] being used for an [`IncomingHttpRequest`].
    pub method: Method,

    /// The path that the [`IncomingHttpRequest`] is being made to.
    pub path: String,

    /// The query parameters of the [`IncomingHttpRequest`].
    pub query: HashMap<String, String>,

    /// The headers of the [`IncomingHttpRequest`].
    pub headers: HashMap<String, String>,

    /// The body of the [`IncomingHttpRequest`].
    pub body: String,

    /// The IP address that the [`IncomingHttpRequest`] is being made from.
    pub remote_addr: String,

    /// Whether the [`IncomingHttpRequest`] is being made by an admin.
    pub authenticated: bool,

    /// The [`User`] making the [`IncomingHttpRequest`].
    pub user: Option<User>
}