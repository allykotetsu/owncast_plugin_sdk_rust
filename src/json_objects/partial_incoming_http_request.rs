use std::collections::HashMap;
use crate::json_objects::user::User;
use crate::prelude::IncomingHttpRequest;

pub struct PartialIncomingHttpRequest {
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub remote_addr: String,
    pub authenticated: bool,
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