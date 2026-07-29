use std::collections::HashMap;
use std::string::FromUtf8Error;
use extism_pdk::http::HttpResponse as ExtismHttpResponse;

pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl TryFrom<ExtismHttpResponse> for HttpResponse {
    type Error = FromUtf8Error;

    fn try_from(extism_http_response: ExtismHttpResponse) -> Result<Self, Self::Error> {
        Ok(HttpResponse {
            status: extism_http_response.status_code(),
            body: String::from_utf8(extism_http_response.body())?,
            headers: extism_http_response.headers().clone()
        })
    }
}