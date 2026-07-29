use extism_pdk::{FromBytes, Json};
use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::method::Method;
use crate::json_objects::user::User;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct IncomingHttpRequest {
    pub method: Method,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub remote_addr: String,
    pub authenticated: bool,
    pub user: Option<User>
}