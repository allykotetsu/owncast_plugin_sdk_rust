use std::collections::HashMap;
use extism_pdk::error;
use serde_json::Error;
use crate::command::command_context::CommandContext;
use crate::command::command_definition::CommandDefinition;
use crate::event_function::{EventFunction, EventFunctionVoid};
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::content_request::ContentRequest;
use crate::json_objects::envelope::Envelope;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_quote::FediverseQuote;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::filter_result::FilterResult;
use crate::json_objects::timer_fire_event::TimerFireEvent;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::manifest::Manifest;
use crate::json_objects::method::Method;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::json_objects::partial_incoming_http_request::PartialIncomingHttpRequest;
use crate::json_objects::sse_connection_event::SseConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;
use crate::json_objects::permission::Permission;
use crate::plugin_state::PluginState;

/// The actual plugin object. This should be immutable and only touched by the library. Contains functions for reading plugin data that is used by the WASM export functions.
pub struct Plugin {
    // Manifest
    pub(crate) manifest: Manifest,

    // Init
    pub(crate) on_init: Option<fn(&mut PluginState)>,

    // Events
    pub(crate) on_chat_message: Vec<EventFunctionVoid<ChatMessage>>,
    pub(crate) on_chat_user_joined: Vec<EventFunctionVoid<User>>,
    pub(crate) on_chat_user_parted: Vec<EventFunctionVoid<User>>,
    pub(crate) on_chat_user_renamed: Vec<EventFunctionVoid<ChatUserRename>>,
    pub(crate) on_message_moderated: Vec<EventFunctionVoid<ChatMessageModeration>>,

    pub(crate) on_stream_started: Vec<EventFunctionVoid<StreamStarted>>,
    pub(crate) on_stream_stopped: Vec<EventFunctionVoid<StreamStopped>>,
    pub(crate) on_stream_title_changed: Vec<EventFunctionVoid<StreamTitleChange>>,

    pub(crate) on_sse_connect: Vec<EventFunctionVoid<SseConnectionEvent>>,
    pub(crate) on_sse_disconnect: Vec<EventFunctionVoid<SseConnectionEvent>>,

    pub(crate) on_tick: Vec<EventFunctionVoid<TickEvent>>,

    pub(crate) on_fediverse: Vec<EventFunctionVoid<HashMap<String, String>>>,
    pub(crate) on_fediverse_follow: Vec<EventFunctionVoid<FediverseEngagement>>,
    pub(crate) on_fediverse_like: Vec<EventFunctionVoid<FediverseTargetedEngagement>>,
    pub(crate) on_fediverse_repost: Vec<EventFunctionVoid<FediverseTargetedEngagement>>,
    pub(crate) on_fediverse_quote: Vec<EventFunctionVoid<FediverseQuote>>,
    pub(crate) on_fediverse_mention: Vec<EventFunctionVoid<FediverseInboundPost>>,
    pub(crate) on_fediverse_reply: Vec<EventFunctionVoid<FediverseInboundPost>>,

    pub(crate) on: Vec<(String, Box<dyn Fn(&mut PluginState, &str) -> Result<(), Error>>)>,

    // Filter
    pub(crate) filter_chat_message: Vec<(u8, EventFunction<ChatMessage, FilterResult>)>,

    // HTTP
    pub(crate) on_http_request: HashMap<(Method, String), EventFunction<PartialIncomingHttpRequest, OutgoingHttpResponse>>,

    // Auth Check
    pub(crate) on_auth_check: Option<EventFunction<User, AuthCheckResult>>,

    // Tab Content
    pub(crate) on_tab_content: HashMap<String, EventFunction<Option<User>, String>>,

    // Page Content
    pub(crate) on_page_content: HashMap<String, EventFunction<Option<User>, String>>,

    // Page Styles
    pub(crate) on_page_styles: Option<fn(&mut PluginState) -> String>,

    // Page Scripts
    pub(crate) on_page_scripts: Option<fn(&mut PluginState) -> String>,

    // Commands
    pub(crate) commands: HashMap<String, CommandDefinition>
}

impl Plugin {
    pub fn is_permitted(&self, permission: Permission) -> bool {
        self.manifest.permissions.contains(&permission)
    }

    pub fn dispatch_init(&self, plugin_state: &mut PluginState) {
        if let Some(on_init) = self.on_init {
            on_init(plugin_state);
        }
    }

    pub fn get_manifest(&self) -> Manifest {
        self.manifest.clone()
    }

    pub fn dispatch_event(&self, plugin_state: &mut PluginState, event: Envelope) {
        match event {
            Envelope::ChatMessageReceived(payload) => {
                for func in &self.on_chat_message { func(plugin_state, &payload); }
            }
            Envelope::ChatUserJoined(payload) => {
                for func in &self.on_chat_user_joined { func(plugin_state, &payload); }
            }
            Envelope::ChatUserParted(payload) => {
                for func in &self.on_chat_user_parted { func(plugin_state, &payload); }
            }
            Envelope::ChatUserRenamed(payload) => {
                for func in &self.on_chat_user_renamed { func(plugin_state, &payload); }
            }
            Envelope::ChatMessageModerated(payload) => {
                for func in &self.on_message_moderated { func(plugin_state, &payload); }
            }

            Envelope::StreamStarted(payload) => {
                for func in &self.on_stream_started { func(plugin_state, &payload); }
            }
            Envelope::StreamStopped(payload) => {
                for func in &self.on_stream_stopped { func(plugin_state, &payload); }
            }
            Envelope::StreamTitleChanged(payload) => {
                for func in &self.on_stream_title_changed { func(plugin_state, &payload); }
            }

            Envelope::SseConnect(payload) => {
                for func in &self.on_sse_connect { func(plugin_state, &payload); }
            }
            Envelope::SseDisconnect(payload) => {
                for func in &self.on_sse_disconnect { func(plugin_state, &payload);}
            }

            Envelope::Tick(payload) => {
                for func in &self.on_tick { func(plugin_state, &payload); }
            }

            Envelope::FediverseActivity(payload) => {
                for func in &self.on_fediverse { func(plugin_state, &payload); }
            }
            Envelope::FediverseFollow(payload) => {
                for func in &self.on_fediverse_follow { func(plugin_state, &payload); }
            }
            Envelope::FediverseLike(payload) => {
                for func in &self.on_fediverse_like { func(plugin_state, &payload); }
            }
            Envelope::FediverseRepost(payload) => {
                for func in &self.on_fediverse_repost { func(plugin_state, &payload); }
            }
            Envelope::FediverseQuote(payload) => {
                for func in &self.on_fediverse_quote { func(plugin_state, &payload); }
            }

            Envelope::FediverseMention(payload) => {
                for func in &self.on_fediverse_mention { func(plugin_state, &payload); }
            }
            Envelope::FediverseReply(payload) => {
                for func in &self.on_fediverse_reply { func(plugin_state, &payload); }
            }

            Envelope::ChatCommand(payload) => {
                if let Some(command_definition) = self.commands.get(&payload.command) {
                    (command_definition.run)(plugin_state, &CommandContext {
                        user: payload.message.user.clone(),
                        msg: payload.message,
                        command: payload.command,
                        invoked_as: payload.invoked_as,
                        args: payload.args,
                        arg_string: payload.arg_string,
                    })
                }
            }
            Envelope::TimerFire(TimerFireEvent { id }) => {
                plugin_state.fire_timer(id);
            }

            Envelope::Custom { event_type, payload } => {
                for (other_name, func) in &self.on {
                    if event_type == *other_name {
                        if let Err(err) = func(plugin_state, payload.as_str()) {
                            error!("{err}");
                        }
                    }
                }
            }
        }
    }

    pub fn dispatch_filter(&self, plugin_state: &mut PluginState, mut msg: ChatMessage) -> FilterResult {
        let mut changed = false;

        for (_, filter_chat_message) in &self.filter_chat_message {
            match filter_chat_message(plugin_state, &msg) {
                FilterResult::Pass => {
                    continue;
                }
                FilterResult::Modify { payload } => {
                    changed = true;
                    msg = payload;
                }
                FilterResult::Drop { reason } => {
                    return FilterResult::Drop { reason }
                }
            }
        }

        if changed {
            FilterResult::Modify { payload: msg }
        } else {
            FilterResult::Pass
        }
    }

    pub fn dispatch_http_request(&self, plugin_state: &mut PluginState, incoming_http_request: IncomingHttpRequest) -> OutgoingHttpResponse {
        if self.on_http_request.is_empty() {
            // If plugin does not listen for HTTP requests, then return 404.
            OutgoingHttpResponse::new(404)
        } else {
            if let Some(func) = self.on_http_request.get(&(incoming_http_request.method.clone(), incoming_http_request.path.clone())) {
                func(plugin_state, &incoming_http_request.into()).clean_clone()
            } else {
                OutgoingHttpResponse::new(200)
            }
        }
    }

    pub fn dispatch_tab_content(&self, plugin_state: &mut PluginState, ContentRequest { slug, user }: ContentRequest) -> Option<String> {
        Some(self.on_tab_content.get(&slug)?(plugin_state, &user))
    }

    pub fn dispatch_page_content(&self, plugin_state: &mut PluginState, ContentRequest { slug, user }: ContentRequest) -> Option<String> {
        Some(self.on_page_content.get(&slug)?(plugin_state, &user))
    }

    pub fn dispatch_page_styles(&self, plugin_state: &mut PluginState) -> Option<String> {
        Some(self.on_page_styles.clone()?(plugin_state))
    }

    pub fn dispatch_page_scripts(&self, plugin_state: &mut PluginState) -> Option<String> {
        Some(self.on_page_scripts.clone()?(plugin_state))
    }

    pub fn dispatch_auth_check(&self, plugin_state: &mut PluginState, AuthCheckRequest { user }: AuthCheckRequest) -> Option<AuthCheckResult> {
        Some(self.on_auth_check?(plugin_state, &user))
    }
}
