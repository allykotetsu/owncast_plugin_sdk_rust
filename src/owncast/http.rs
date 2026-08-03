use extism_pdk::{http, HttpRequest, SharedFnResult};
use crate::json_objects::http_request_opts::HttpRequestOpts;
use crate::json_objects::http_response::HttpResponse;

// TODO haven't verified as working.
pub fn fetch(url: &str, opts: Option<HttpRequestOpts>) -> SharedFnResult<HttpResponse> {
    let opts = opts.unwrap_or(HttpRequestOpts::new());
    let mut http_request = HttpRequest::new(url);
    if let Some(method) = opts.method {
        http_request = http_request.with_method(method);
    }
    if let Some(headers) = opts.headers {
        for (key, value) in headers {
            http_request = http_request.with_header(key, value);
        }
    }
    http::request(&http_request, opts.body).map(|x| Ok(HttpResponse::try_from(x)?))?
}