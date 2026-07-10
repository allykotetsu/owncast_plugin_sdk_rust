use std::collections::HashMap;
use serde::Deserialize;
use crate::objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingHttpRequest {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub remote_addr: String,
    pub authenticated: bool,
    pub user: Option<User>
}