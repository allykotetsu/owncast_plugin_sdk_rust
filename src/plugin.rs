use std::collections::HashMap;
use serde_json::Error;
use crate::command::command_data::CommandDefinition;
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command::Command;
use crate::json_objects::content_request::ContentRequest;
use crate::json_objects::event::Event;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::filter::Filter;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::manifest::Manifest;
use crate::json_objects::notify::Notify;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::subscriptions::Subscriptions;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;

/// The actual plugin object. This should be immutable and only touched by the library. Contains functions for reading plugin data that is used by the WASM export functions.
pub(crate) struct Plugin<'a> {
    // Events
    pub(crate) on_chat_message: Vec<Box<dyn Fn(&ChatMessage)>>,
    pub(crate) on_chat_user_joined: Vec<Box<dyn Fn(&User)>>,
    pub(crate) on_chat_user_parted: Vec<Box<dyn Fn(&User)>>,
    pub(crate) on_chat_user_renamed: Vec<Box<dyn Fn(&ChatUserRename)>>,
    pub(crate) on_message_moderated: Vec<Box<dyn Fn(&ChatMessageModeration)>>,

    pub(crate) on_stream_started: Vec<Box<dyn Fn(&StreamStarted)>>,
    pub(crate) on_stream_stopped: Vec<Box<dyn Fn(&StreamStopped)>>,
    pub(crate) on_stream_title_changed: Vec<Box<dyn Fn(&StreamTitleChange)>>,

    pub(crate) on_sse_connect: Vec<Box<dyn Fn(&SSEConnectionEvent)>>,
    pub(crate) on_sse_disconnect: Vec<Box<dyn Fn(&SSEConnectionEvent)>>,

    pub(crate) on_tick: Vec<Box<dyn Fn(&TickEvent)>>,

    pub(crate) on_fediverse: Vec<Box<dyn Fn(&HashMap<String, String>)>>,
    pub(crate) on_fediverse_follow: Vec<Box<dyn Fn(&FediverseEngagement)>>,
    pub(crate) on_fediverse_like: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    pub(crate) on_fediverse_repost: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    pub(crate) on_fediverse_quote: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    pub(crate) on_fediverse_mention: Vec<Box<dyn Fn(&FediverseInboundPost)>>,
    pub(crate) on_fediverse_reply: Vec<Box<dyn Fn(&FediverseInboundPost)>>,

    pub(crate) on: Vec<(String, Box<dyn Fn(&str) -> Result<(), Error>>)>,

    // Filter
    pub(crate) filter_chat_message: Vec<(u8, Box<dyn Fn(&ChatMessage) -> FilterResult>)>,

    // HTTP
    pub(crate) on_http_request: HashMap<(Method, String), Box<&'a dyn Fn(&IncomingHttpRequest) -> OutgoingHttpResponse>>,

    // Auth Check
    pub(crate) on_auth_check: Vec<Box<dyn Fn(&AuthCheckRequest) -> AuthCheckResult>>,

    // Tab Content
    pub(crate) on_tab_content: Vec<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Content
    pub(crate) on_page_content: Vec<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Styles
    pub(crate) on_page_styles: Vec<Box<dyn Fn() -> String>>,

    // Page Scripts
    pub(crate) on_page_scripts: Vec<Box<dyn Fn() -> String>>,

    // Commands
    pub(crate) commands: HashMap<String, CommandDefinition<'a>>
}

impl<'a> Plugin<'a> {
    fn get_subscriptions(&self) -> Subscriptions {
        // Construct notifications.
        let mut notify = vec![];
        if !self.on_chat_message.is_empty() {
            notify.push(Notify { event: "".to_string() });
        }

        // Construct filter.
        let mut filter = vec![];
        if let Some(&(priority, _)) = self.filter_chat_message.first() {
            filter.push(Filter {
                event: "".to_string(),
                priority
            })
        }

        Subscriptions {
            notify,
            filter
        }
    }

    fn get_commands(&self) -> Vec<Command> {
        self.commands.values().map(|CommandDefinition { command, .. }| command.clone()).collect()
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
            Event::ChatMessageReceived(payload) => {
                for func in &self.on_chat_message {
                    func(&payload);
                }
            }

            Event::ChatUserJoined(payload) => {
                for func in &self.on_chat_user_joined {
                    func(&payload);
                }
            }

            Event::ChatUserParted(payload) => {
                for func in &self.on_chat_user_parted {
                    func(&payload);
                }
            }

            Event::ChatUserRenamed(payload) => {
                for func in &self.on_chat_user_renamed {
                    func(&payload);
                }
            }

            Event::ChatMessageModerated(payload) => {
                for func in &self.on_message_moderated {
                    func(&payload);
                }
            }

            Event::StreamStarted(payload) => {
                for func in &self.on_stream_started {
                    func(&payload);
                }
            }

            Event::StreamStopped(payload) => {
                for func in &self.on_stream_stopped {
                    func(&payload);
                }
            }

            Event::StreamTitleChanged(payload) => {
                for func in &self.on_stream_title_changed {
                    func(&payload);
                }
            }

            Event::SseConnect(payload) => {
                for func in &self.on_sse_connect {
                    func(&payload);
                }
            }

            Event::SseDisconnect(payload) => {
                for func in &self.on_sse_disconnect {
                    func(&payload);
                }
            }

            Event::Tick(payload) => {
                for func in &self.on_tick {
                    func(&payload);
                }
            }

            Event::FediverseActivity(payload) => {
                for func in &self.on_fediverse {
                    func(&payload);
                }
            }

            Event::FediverseFollow(payload) => {
                for func in &self.on_fediverse_follow {
                    func(&payload);
                }
            }

            Event::FediverseLike(payload) => {
                for func in &self.on_fediverse_like {
                    func(&payload);
                }
            }

            Event::FediverseRepost(payload) => {
                for func in &self.on_fediverse_repost {
                    func(&payload);
                }
            }

            Event::FediverseQuote(payload) => {
                for func in &self.on_fediverse_quote {
                    func(&payload);
                }
            }

            Event::FediverseMention(payload) => {
                for func in &self.on_fediverse_mention {
                    func(&payload);
                }
            }

            Event::FediverseReply(payload) => {
                for func in &self.on_fediverse_reply {
                    func(&payload);
                }
            },

            Event::Custom(name, payload) => {
                for (other_name, func) in &self.on {
                    if name == *other_name {
                        if let Err(err) = func(payload.as_str()) {
                            println!("{err}");
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn on_filter(&self, msg: ChatMessage) -> FilterResult {
        let mut body = String::new();

        for (_, filter_chat_message) in &self.filter_chat_message {
            match filter_chat_message(&msg) {
                FilterResult::Pass => {
                    continue;
                }
                FilterResult::Modify(new) => {
                    body = new;
                }
                FilterResult::Drop(reason) => {
                    return FilterResult::Drop(reason)
                }
            }
        }

        if body.is_empty() {
            FilterResult::Pass
        } else {
            FilterResult::Modify(body)
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
                    let outgoing_http_response = func(&incoming_http_request);

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

    pub(crate) fn has_page_styles(&self) -> bool {
        false
    }

    pub(crate) fn has_page_scripts(&self) -> bool {
        false
    }

    pub(crate) fn has_auth_check(&self) -> bool {
        false
    }
}