use std::collections::HashMap;
use crate::command::command_data::CommandData;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::command::Command;
use crate::json_objects::event::Event;
use crate::json_objects::filter::Filter;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::manifest::Manifest;
use crate::json_objects::notify::Notify;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::json_objects::subscriptions::Subscriptions;

/// The actual plugin object. This should be immutable and only touched by the library. Contains functions for reading plugin data that is used by the WASM export functions.
pub(crate) struct Plugin<'a> {
    // Events
    pub(crate) on_chat_message: Vec<Box<dyn Fn(ChatMessage)>>,
    pub(crate) on: Vec<(String, Box<dyn Fn(&str)>)>,

    // Filter
    pub(crate) filter_chat_message: Vec<(u8, Box<dyn Fn(ChatMessage) -> FilterResult>)>,

    // HTTP
    pub(crate) on_http_request: HashMap<(Method, String), Box<&'a dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>,

    // Commands
    pub(crate) commands: HashMap<String, CommandData<'a>>
}

impl<'a> Plugin<'a> {
    fn get_subscriptions(&self) -> Subscriptions {
        // Construct notifications.
        let mut notify = vec![];
        if !self.on_chat_message.is_empty() {
            notify.push(Notify { event: Event::ChatMessageReceived(None) });
        }

        // Construct filter.
        let mut filter = vec![];
        if let Some(&(priority, _)) = self.filter_chat_message.first() {
            filter.push(Filter {
                event: Event::ChatMessageReceived(None),
                priority
            })
        }

        Subscriptions {
            notify,
            filter
        }
    }

    fn get_commands(&self) -> Vec<Command> {
        self.commands.values().map(|CommandData { command, .. }| command.clone()).collect()
    }

    // TODO
    pub(crate) fn get_manifest(&self) -> Manifest {
        let subscriptions = self.get_subscriptions();
        let commands = self.get_commands();
        Manifest { subscriptions, commands }
    }

    // TODO
    pub(crate) fn on_event(&self, event: Event) {
        match event {
            Event::ChatMessageReceived(Some(payload)) => {
                for on_chat_message in &self.on_chat_message {
                    on_chat_message(payload.clone());
                }
            }

            Event::ChatUserJoined(_) => {}
            Event::ChatUserParted(_) => {}
            Event::ChatUserRenamed => {}
            Event::ChatMessageModerated => {}
            Event::StreamStarted => {}
            Event::StreamStopped => {}
            Event::StreamTitleChanged => {}
            Event::SseConnect => {}
            Event::SseDisconnect => {}
            Event::Tick => {}
            Event::FediverseFollow => {}
            Event::FediverseLike => {}
            Event::FediverseRepost => {}
            Event::FediverseMention => {}
            Event::FediverseReply => {},
            Event::Custom(name) => {
                for (other_name, _func) in &self.on {
                    if name == *other_name {
                        // _func
                    }
                }
            }
            _ => {}
        }
    }

    pub(crate) fn on_filter(&self, mut msg: ChatMessage) -> FilterResult {
        let mut modified = false;

        for (_, filter_chat_message) in &self.filter_chat_message {
            match filter_chat_message(msg.clone()) {
                FilterResult::Pass => {
                    continue;
                }
                FilterResult::Modify(new) => {
                    msg = new;
                    modified = true;
                }
                FilterResult::Drop(reason) => {
                    return FilterResult::Drop(reason)
                }
            }
        }

        if modified {
            FilterResult::Modify(msg)
        } else {
            FilterResult::Pass
        }
    }

    pub(crate) fn on_http_request(&self, incoming_http_request: IncomingHttpRequest) -> OutgoingHttpResponse {
        if self.on_http_request.is_empty() {
            OutgoingHttpResponse {
                status: Some(404),
                headers: None,
                body: None
            }
        } else {
            if let Ok(method) = Method::try_from(&incoming_http_request.method) {
                if let Some(func) = self.on_http_request.get(&(method, incoming_http_request.path.clone())) {
                    let outgoing_http_response = func(incoming_http_request);

                    OutgoingHttpResponse {
                        status: Some(outgoing_http_response.status.unwrap_or(200)),
                        headers: outgoing_http_response.headers,
                        body: outgoing_http_response.body
                    }
                } else {
                    OutgoingHttpResponse {
                        status: Some(200),
                        headers: None,
                        body: None
                    }
                }
            } else {
                OutgoingHttpResponse {
                    status: Some(400),
                    headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                    body: Some(format!("Unable to parse request method {}.", incoming_http_request.method))
                }
            }
        }
    }
}