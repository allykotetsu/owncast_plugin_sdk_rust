use std::collections::HashMap;
use crate::json_objects::method::Method;

#[derive(Clone, Debug)]
pub struct HttpRequestOpts {
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>
}

impl HttpRequestOpts {
    pub fn new() -> Self {
        Self {
            method: None,
            body: None,
            headers: None
        }
    }

    pub fn with_method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        if let Some(ref mut headers) = self.headers {
            headers.insert(key.to_string(), value.to_string());
        } else {
            self.headers = Some(HashMap::from([(key.to_string(), value.to_string())]));
        }
        self
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
}