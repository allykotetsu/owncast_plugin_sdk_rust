use std::collections::HashMap;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::plugin_builder::CommandData;

pub(crate) struct Plugin<'a> {
    pub(crate) on_chat_message: Vec<Box<dyn Fn(&str)>>,
    pub(crate) on_http_request: HashMap<(Method, String), Box<&'a dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>,
    pub(crate) on: HashMap<String, Box<dyn Fn(&str)>>,

    pub(crate) commands: HashMap<String, CommandData>
}