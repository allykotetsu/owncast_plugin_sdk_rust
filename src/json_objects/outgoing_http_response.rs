use extism_pdk::{ToBytes, Json};
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct OutgoingHttpResponse {
    pub(crate) status: Option<u16>,
    pub(crate) headers: Option<HashMap<String, String>>,
    pub(crate) body: Option<String>
}