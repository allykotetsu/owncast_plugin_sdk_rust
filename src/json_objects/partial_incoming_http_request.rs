use std::collections::HashMap;
use crate::json_objects::user::User;
use crate::prelude::IncomingHttpRequest;

// TODO methods for converting query, headers, and body to structs.

/// An object representing an incoming HTTP request without method or path info.
#[derive(Clone, Debug)]
pub struct PartialIncomingHttpRequest {
    /// The query parameters of the [`PartialIncomingHttpRequest`].
    pub query: HashMap<String, String>,

    /// The headers of the [`PartialIncomingHttpRequest`].
    pub headers: HashMap<String, String>,

    /// The body of the [`PartialIncomingHttpRequest`].
    pub body: String,

    /// The IP address that the [`PartialIncomingHttpRequest`] is being made from.
    pub remote_addr: String,

    /// Whether the [`PartialIncomingHttpRequest`] is being made by an admin.
    pub authenticated: bool,

    /// The [`User`] making the [`PartialIncomingHttpRequest`].
    pub user: Option<User>
}

impl From<IncomingHttpRequest> for PartialIncomingHttpRequest {
    fn from(incoming_http_request: IncomingHttpRequest) -> Self {
        Self {
            query: incoming_http_request.query,
            headers: incoming_http_request.headers,
            body: incoming_http_request.body,
            remote_addr: incoming_http_request.remote_addr,
            authenticated: incoming_http_request.authenticated,
            user: incoming_http_request.user
        }
    }
}