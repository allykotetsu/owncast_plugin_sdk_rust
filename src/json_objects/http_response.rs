use std::collections::HashMap;
use std::string::FromUtf8Error;
use extism_pdk::http::HttpResponse as ExtismHttpResponse;
use crate::json_objects::status::Status;

/// The response to an Ok [`owncast::http::fetch`].
#[derive(Clone, Debug)]
pub struct HttpResponse {
    /// The status of the [`HttpResponse`].
    pub status: Status,

    /// The header hashmap of the [`HttpResponse`].
    pub headers: HashMap<String, String>,

    /// The body of the [`HttpResponse`].
    pub body: String
}

impl TryFrom<ExtismHttpResponse> for HttpResponse {
    type Error = FromUtf8Error;

    fn try_from(extism_http_response: ExtismHttpResponse) -> Result<Self, Self::Error> {
        Ok(HttpResponse {
            status: extism_http_response.status_code().into(),
            body: String::from_utf8(extism_http_response.body())?,
            headers: extism_http_response.headers().clone()
        })
    }
}