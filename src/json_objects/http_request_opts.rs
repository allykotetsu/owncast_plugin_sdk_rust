use std::collections::HashMap;
use crate::json_objects::method::Method;

#[derive(Clone, Debug)]
pub struct HttpRequestOpts {
    pub method: Option<Method>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>
}

impl HttpRequestOpts {
    pub(crate) fn empty() -> Self {
        Self {
            method: None,
            body: None,
            headers: None
        }
    }
}