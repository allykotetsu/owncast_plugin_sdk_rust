use extism_pdk::{ToBytes, Json};
use std::collections::HashMap;
use serde::Serialize;

const CONTENT_TYPE: &str = "content-type";

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct OutgoingHttpResponse {
    pub status: Option<u16>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>
}

impl OutgoingHttpResponse {
    pub fn new(status: u16) -> Self {
        Self {
            status: Some(status),
            headers: None,
            body: None
        }
    }

    pub fn text_html(status: u16, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "text/html".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn text_plain(status: u16, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "text/plain".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn application_json(status: u16, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "application/json".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn application_x_www_form_urlencoded(status: u16, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "application/x-www-form-urlencoded".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new())
        }
        self.headers.as_mut().unwrap().insert(key.to_string(), value.to_string());
        self
    }

    pub(crate) fn clean_clone(&self) -> Self {
        OutgoingHttpResponse {
            status: Some(self.status.unwrap_or(200)),
            headers: self.headers.clone(),
            body: self.body.clone()
        }
    }
}