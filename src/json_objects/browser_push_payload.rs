use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct BrowserPushPayload {
    pub title: String,
    pub body: Option<String>,
    pub url: Option<String>
}

impl Into<BrowserPushPayload> for &str {
    fn into(self) -> BrowserPushPayload {
        BrowserPushPayload {
            title: self.to_string(),
            body: None,
            url: None
        }
    }
}