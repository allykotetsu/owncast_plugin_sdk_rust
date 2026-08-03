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

impl BrowserPushPayload {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            body: None,
            url: None,
        }
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
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