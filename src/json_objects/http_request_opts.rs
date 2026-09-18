use std::collections::HashMap;
use crate::json_objects::method::Method;

/// The options passed to an HTTP request.
#[derive(Clone, Debug)]
pub struct HttpRequestOpts {
    /// The [`Method`] being used for the HTTP request.
    pub method: Option<Method>,

    /// A hashmap of headers to use for the HTTP request.
    pub headers: HashMap<String, String>,

    /// The body of the HTTP request.
    pub body: Option<String>
}

impl HttpRequestOpts {
    pub fn new() -> Self {
        Self {
            method: None,
            body: None,
            headers: HashMap::new()
        }
    }

    pub fn with_method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
}