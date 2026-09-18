use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// An object representing a notification sent through the browser.
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct BrowserPushPayload {
    /// The title of the [`BrowserPushPayload`].
    pub title: String,

    /// The body of the [`BrowserPushPayload`].
    pub body: Option<String>,

    /// The URL of the [`BrowserPushPayload`].
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