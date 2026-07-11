use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingHttpRequest {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) query: HashMap<String, String>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: String,
    pub(crate) remote_addr: String,
    pub(crate) authenticated: bool,
    pub(crate) user: Option<User>
}