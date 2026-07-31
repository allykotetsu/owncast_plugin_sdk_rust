use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct BrowserPushPayload {
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>
}

impl From<&str> for BrowserPushPayload {
    fn from(s: &str) -> BrowserPushPayload {
        BrowserPushPayload {
            title: s.to_string(),
            body: None,
            url: None
        }
    }
}