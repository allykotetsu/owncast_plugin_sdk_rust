use std::collections::HashMap;
use crate::method::Method;
use crate::objects::incoming_http_request::IncomingHttpRequest;
use crate::objects::outgoing_http_response::OutgoingHttpResponse;
use crate::plugin::Plugin;

pub struct PluginBuilder {
    _on_chat_message: Vec<Box<dyn Fn(&str)>>,
    _on_http_request: HashMap<(Method, String), Box<dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>
}

impl PluginBuilder {
    pub(crate) fn new() -> Self {
        Self {
            _on_chat_message: vec![],
            _on_http_request: HashMap::new()
        }
    }

    pub fn on_chat_message<F: Fn(&str) -> () + 'static>(&mut self, f: F) {
        self._on_chat_message.push(Box::new(f));
    }

    pub fn on_http_request<F: Fn(IncomingHttpRequest) -> OutgoingHttpResponse + 'static>(&mut self, method: Method, path: &str, f: F) -> Result<(), ()> {
        match self._on_http_request.insert((method, path.to_string()), Box::new(f)) {
            Some(_) => Ok(()),
            None => Err(())
        }
    }
}

impl Into<Plugin> for PluginBuilder {
    fn into(self) -> Plugin {
        Plugin {
            on_chat_message: self._on_chat_message,
            on_http_request: self._on_http_request
        }
    }
}