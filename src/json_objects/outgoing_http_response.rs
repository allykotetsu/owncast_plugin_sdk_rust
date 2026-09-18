use extism_pdk::{ToBytes, Json};
use std::collections::HashMap;
use serde::Serialize;
use serde_json::Error;
use crate::json_objects::status::Status;

// TODO list all headers
const CONTENT_TYPE: &str = "content-type";
const LOCATION: &str = "Location";

/// An object representing a response to an [`IncomingHttpRequest`],
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct OutgoingHttpResponse {
    /// The status of the [`OutgoingHttpResponse`]. Defaults to 200 if omitted.
    pub status: Option<Status>,

    /// The headers of the [`OutgoingHttpResponse`].
    pub headers: Option<HashMap<String, String>>,

    /// The body of the [`OutgoingHttpResponse`].
    pub body: Option<String>
}

impl OutgoingHttpResponse {
    /// Create a new OutgoingHttpResponse object with a status code.
    ///
    /// * `status` - What [`Status`] code the HTTP response uses.
    ///
    /// # Examples
    ///
    /// ```
    /// # use owncast_plugin_sdk_rust::json_objects::status::Status;
    /// # use owncast_plugin_sdk_rust::prelude::*;
    ///
    /// # fn main() {
    /// let outgoing_http_response = OutgoingHttpResponse::new(Status::NotFound);
    /// # }
    /// ```
    pub fn new(status: Status) -> Self {
        Self {
            status: Some(status),
            headers: None,
            body: None
        }
    }

    pub fn redirect_found(url: &str) -> Self {
        Self {
            status: Some(Status::Found),
            headers: Some(HashMap::from([
                (LOCATION.to_string(), url.to_string())
            ])),
            body: None
        }
    }

    pub fn text_html(status: Status, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "text/html".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn text_plain(status: Status, body: &str) -> Self {
        Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "text/plain".to_string())
            ])),
            body: Some(body.to_string())
        }
    }

    pub fn application_json(status: Status, body: impl Serialize) -> Result<Self, Error> {
        Ok(Self {
            status: Some(status),
            headers: Some(HashMap::from([
                (CONTENT_TYPE.to_string(), "application/json".to_string())
            ])),
            body: Some(serde_json::to_string(&body)?)
        })
    }

    pub fn application_x_www_form_urlencoded(status: Status, body: &str) -> Self {
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
            status: match &self.status {
                None => Some(Status::Ok),
                x => x.clone()
            },
            headers: self.headers.clone(),
            body: self.body.clone()
        }
    }
}