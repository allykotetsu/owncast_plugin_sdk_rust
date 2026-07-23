use std::collections::HashMap;
use serde_json::Error;
use crate::command::command_context::CommandContext;
use crate::command::command_definition::CommandDefinition;
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::content_request::ContentRequest;
use crate::json_objects::event::Event;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::manifest::Manifest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;
use crate::permission::Permission;

/// The actual plugin object. This should be immutable and only touched by the library. Contains functions for reading plugin data that is used by the WASM export functions.
pub struct Plugin<'a> {
    // Manifest
    pub(crate) manifest: Manifest,

    // Events
    pub(crate) on_chat_message: Vec<fn(&ChatMessage)>,
    pub(crate) on_chat_user_joined: Vec<fn(&User)>,
    pub(crate) on_chat_user_parted: Vec<fn(&User)>,
    pub(crate) on_chat_user_renamed: Vec<fn(&ChatUserRename)>,
    pub(crate) on_message_moderated: Vec<fn(&ChatMessageModeration)>,

    pub(crate) on_stream_started: Vec<fn(&StreamStarted)>,
    pub(crate) on_stream_stopped: Vec<fn(&StreamStopped)>,
    pub(crate) on_stream_title_changed: Vec<fn(&StreamTitleChange)>,

    pub(crate) on_sse_connect: Vec<fn(&SSEConnectionEvent)>,
    pub(crate) on_sse_disconnect: Vec<fn(&SSEConnectionEvent)>,

    pub(crate) on_tick: Vec<fn(&TickEvent)>,

    pub(crate) on_fediverse: Vec<fn(&HashMap<String, String>)>,
    pub(crate) on_fediverse_follow: Vec<fn(&FediverseEngagement)>,
    pub(crate) on_fediverse_like: Vec<fn(&FediverseTargetedEngagement)>,
    pub(crate) on_fediverse_repost: Vec<fn(&FediverseTargetedEngagement)>,
    pub(crate) on_fediverse_quote: Vec<fn(&FediverseTargetedEngagement)>,
    pub(crate) on_fediverse_mention: Vec<fn(&FediverseInboundPost)>,
    pub(crate) on_fediverse_reply: Vec<fn(&FediverseInboundPost)>,

    pub(crate) on: Vec<(String, Box<dyn Fn(&str) -> Result<(), Error>>)>,

    // Filter
    pub(crate) filter_chat_message: Vec<(u8, fn(&ChatMessage) -> FilterResult)>,

    // HTTP
    pub(crate) on_http_request: HashMap<(Method, String), &'a fn(&IncomingHttpRequest) -> OutgoingHttpResponse>,

    // Auth Check
    pub(crate) on_auth_check: Option<fn(&AuthCheckRequest) -> AuthCheckResult>,

    // Tab Content
    pub(crate) on_tab_content: HashMap<String, fn(&ContentRequest) -> String>,

    // Page Content
    pub(crate) on_page_content: HashMap<String, fn(&ContentRequest) -> String>,

    // Page Styles
    pub(crate) on_page_styles: Option<fn() -> String>,

    // Page Scripts
    pub(crate) on_page_scripts: Option<fn() -> String>,

    // Commands
    pub(crate) commands: HashMap<String, CommandDefinition>
}

impl<'a> Plugin<'a> {
    pub fn is_permitted(&self, permission: Permission) -> bool {
        self.manifest.permissions.contains(&permission)
    }

    pub fn get_manifest(&self) -> Manifest {
        self.manifest.clone()
    }

    pub fn dispatch_event(&self, event: Event) {
        match event {
            Event::ChatMessageReceived(payload) => {
                for func in &self.on_chat_message { func(&payload); }
            }
            Event::ChatUserJoined(payload) => {
                for func in &self.on_chat_user_joined { func(&payload); }
            }
            Event::ChatUserParted(payload) => {
                for func in &self.on_chat_user_parted { func(&payload); }
            }
            Event::ChatUserRenamed(payload) => {
                for func in &self.on_chat_user_renamed { func(&payload); }
            }
            Event::ChatMessageModerated(payload) => {
                for func in &self.on_message_moderated { func(&payload); }
            }

            Event::StreamStarted(payload) => {
                for func in &self.on_stream_started { func(&payload); }
            }
            Event::StreamStopped(payload) => {
                for func in &self.on_stream_stopped { func(&payload); }
            }
            Event::StreamTitleChanged(payload) => {
                for func in &self.on_stream_title_changed { func(&payload); }
            }

            Event::SSEConnect(payload) => {
                for func in &self.on_sse_connect { func(&payload); }
            }
            Event::SSEDisconnect(payload) => {
                for func in &self.on_sse_disconnect { func(&payload);}
            }

            Event::Tick(payload) => {
                for func in &self.on_tick { func(&payload); }
            }

            Event::FediverseActivity(payload) => {
                for func in &self.on_fediverse { func(&payload); }
            }
            Event::FediverseFollow(payload) => {
                for func in &self.on_fediverse_follow { func(&payload); }
            }
            Event::FediverseLike(payload) => {
                for func in &self.on_fediverse_like { func(&payload); }
            }
            Event::FediverseRepost(payload) => {
                for func in &self.on_fediverse_repost { func(&payload); }
            }
            Event::FediverseQuote(payload) => {
                for func in &self.on_fediverse_quote { func(&payload); }
            }

            Event::FediverseMention(payload) => {
                for func in &self.on_fediverse_mention { func(&payload); }
            }
            Event::FediverseReply(payload) => {
                for func in &self.on_fediverse_reply { func(&payload); }
            }

            Event::ChatCommand(payload) => {
                if let Some(command_definition) = self.commands.get(&payload.command) {
                    (command_definition.run)(&CommandContext {
                        user: payload.message.user.clone(),
                        msg: payload.message,
                        command: payload.command,
                        invoked_as: payload.invoked_as,
                        args: payload.args,
                        arg_string: payload.arg_string,
                    })
                }
            }
            Event::TimerFire() => {
                // TODO
            }

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

    pub fn dispatch_filter(&self, msg: ChatMessage) -> FilterResult {
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

    pub fn dispatch_http_request(&self, incoming_http_request: IncomingHttpRequest) -> OutgoingHttpResponse {
        if self.on_http_request.is_empty() {
            // If plugin does not listen for HTTP requests, then return 404.
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
                // If HTTP request method is not a valid type, then return a bad request.
                OutgoingHttpResponse {
                    status: Some(400),
                    headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                    body: Some(format!("Unable to parse request method {}.", incoming_http_request.method))
                }
            }
        }
    }

    pub fn dispatch_tab_content(&self, content_request: ContentRequest) -> Option<String> {
        Some(self.on_tab_content.get(&content_request.slug)?(&content_request))
    }

    pub fn dispatch_page_content(&self, content_request: ContentRequest) -> Option<String> {
        Some(self.on_page_content.get(&content_request.slug)?(&content_request))
    }

    pub fn dispatch_page_styles(&self) -> Option<String> {
        Some(self.on_page_styles.clone()?())
    }

    pub fn dispatch_page_scripts(&self) -> Option<String> {
        Some(self.on_page_scripts.clone()?())
    }

    pub fn dispatch_auth_check(&self, auth_check_request: AuthCheckRequest) -> Option<AuthCheckResult> {
        Some(self.on_auth_check?(&auth_check_request))
    }
}
