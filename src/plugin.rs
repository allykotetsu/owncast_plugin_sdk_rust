use std::collections::HashMap;
use crate::method::Method;
use crate::objects::incoming_http_request::IncomingHttpRequest;
use crate::objects::outgoing_http_response::OutgoingHttpResponse;

pub(crate) struct Plugin {
    pub(crate) on_chat_message: Vec<Box<dyn Fn(&str)>>,
    pub(crate) on_http_request: HashMap<(Method, String), Box<dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>
}