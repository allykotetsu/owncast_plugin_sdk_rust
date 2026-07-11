use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingHttpResponse {
    pub(crate) status: Option<u16>,
    pub(crate) headers: Option<HashMap<String, String>>,
    pub(crate) body: Option<String>
}