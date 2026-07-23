use std::collections::HashMap;
use extism_pdk::config;
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_definition::CommandDefinition;
use crate::errors::{Duplicate, MissingManifest, OutOfBounds};
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command::Command;
use crate::json_objects::content_request::ContentRequest;
use crate::json_objects::event::{CHAT_MESSAGE_MODERATED, CHAT_MESSAGE_RECEIVED, CHAT_USER_JOINED, CHAT_USER_PARTED, CHAT_USER_RENAMED, FEDIVERSE_ACTIVITY, FEDIVERSE_FOLLOW, FEDIVERSE_LIKE, FEDIVERSE_MENTION, FEDIVERSE_QUOTE, FEDIVERSE_REPLY, FEDIVERSE_REPOST, SSE_CONNECT, SSE_DISCONNECT, STREAM_STARTED, STREAM_STOPPED, STREAM_TITLE_CHANGED, TICK};
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
use crate::partial_manifest::PartialManifest;
use crate::plugin::Plugin;

/// The plugin builder that the plugin author uses to add functionality to the plugin. Plugin authors should not instantiate this type on their own (see [`define_plugin!`]).
pub struct PluginBuilder<'a> {
    // Events
    partial_manifest: PartialManifest,

    on_chat_message_: Vec<fn(&ChatMessage)>,
    on_chat_user_joined_: Vec<fn(&User)>,
    on_chat_user_parted_: Vec<fn(&User)>,
    on_chat_user_renamed_: Vec<fn(&ChatUserRename)>,
    on_message_moderated_: Vec<fn(&ChatMessageModeration)>,

    on_stream_started_: Vec<fn(&StreamStarted)>,
    on_stream_stopped_: Vec<fn(&StreamStopped)>,
    on_stream_title_changed_: Vec<fn(&StreamTitleChange)>,

    on_sse_connect_: Vec<fn(&SSEConnectionEvent)>,
    on_sse_disconnect_: Vec<fn(&SSEConnectionEvent)>,

    on_tick_: Vec<fn(&TickEvent)>,

    on_fediverse_: Vec<fn(&HashMap<String, String>)>,
    on_fediverse_follow_: Vec<fn(&FediverseEngagement)>,
    on_fediverse_like_: Vec<fn(&FediverseTargetedEngagement)>,
    on_fediverse_repost_: Vec<fn(&FediverseTargetedEngagement)>,
    on_fediverse_quote_: Vec<fn(&FediverseTargetedEngagement)>,
    on_fediverse_mention_: Vec<fn(&FediverseInboundPost)>,
    on_fediverse_reply_: Vec<fn(&FediverseInboundPost)>,

    on_: Vec<(String, Box<dyn Fn(&str) -> Result<(), Error>>)>,

    // Filter
    filter_chat_message_: Vec<(u8, fn(&ChatMessage) -> FilterResult)>,

    // HTTP
    on_http_request_: HashMap<(Method, String), &'a fn(&IncomingHttpRequest) -> OutgoingHttpResponse>,

    // Auth Check
    on_auth_check_: Option<fn(&AuthCheckRequest) -> AuthCheckResult>,

    // Tab Content
    on_tab_content_: HashMap<String, fn(&ContentRequest) -> String>,

    // Page Content
    on_page_content_: HashMap<String, fn(&ContentRequest) -> String>,

    // Page Styles
    on_page_styles_: Option<fn() -> String>,

    // Page Scripts
    on_page_scripts_: Option<fn() -> String>,

    // Commands
    commands_: HashMap<String, CommandDefinition>
}

impl<'a> PluginBuilder<'a> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(manifest) =  config::get("manifest")? {
            Ok(Self {
                partial_manifest: serde_json::from_str(&manifest)?,

                on_chat_message_: vec![],
                on_chat_user_joined_: vec![],
                on_chat_user_parted_: vec![],
                on_chat_user_renamed_: vec![],
                on_message_moderated_: vec![],
                on_stream_started_: vec![],
                on_stream_stopped_: vec![],
                on_stream_title_changed_: vec![],
                on_sse_connect_: vec![],
                on_sse_disconnect_: vec![],
                on_tick_: vec![],
                on_fediverse_: vec![],
                on_fediverse_follow_: vec![],
                on_fediverse_like_: vec![],
                on_fediverse_repost_: vec![],
                on_fediverse_quote_: vec![],
                on_fediverse_mention_: vec![],
                on_fediverse_reply_: vec![],
                on_: vec![],

                filter_chat_message_: vec![],

                on_http_request_: HashMap::new(),

                on_auth_check_: None,

                on_tab_content_: HashMap::new(),

                on_page_content_: HashMap::new(),

                on_page_styles_: None,

                on_page_scripts_: None,

                commands_: HashMap::new()
            })
        } else {
            Err(Box::new(MissingManifest("Manifest could not be found.".to_string())))
        }
    }

    /// Creates an event hook for when a chat message is sent.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
    ///         owncast_send_chat(&format!("echo {body}"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_chat_message(&mut self, f: fn(&ChatMessage) -> ()) {
        self.on_chat_message_.push(f);
    }

    /// Creates an event hook for when a user joins stream chat.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_chat_user_joined(|ChatUser { display_name, .. }| {
    ///         owncast_send_chat(&format!("Welcome, {display_name}!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_chat_user_joined(&mut self, f: fn(&User) -> ()) {
        self.on_chat_user_joined_.push(f);
    }

    /// Creates an event hook for when a user leaves stream chat.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_chat_user_parted(|ChatUser { display_name, .. }| {
    ///         owncast_send_chat(&format!("Goodbye, {display_name}!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_chat_user_parted(&mut self, f: fn(&User) -> ()) {
        self.on_chat_user_parted_.push(f);
    }

    /// Creates an event hook for when a user leaves stream chat.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_chat_user_renamed(|ChatUserRename { display_name, .. }| {
    ///         owncast_send_chat(&format!("Goodbye, {display_name}!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_chat_user_renamed(&mut self, f: fn(&ChatUserRename) -> ()) {
        self.on_chat_user_renamed_.push(f);
    }

    /// Creates an event hook for when a moderator moderates a message.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_message_moderated(|ChatMessageModeration { message_id, visible, .. }| {
    ///         owncast_send_chat(&format!("A moderator changed {message_id} to {visible}!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_message_moderated(&mut self, f: fn(&ChatMessageModeration) -> ()) {
        self.on_message_moderated_.push(f);
    }

    /// Creates an event hook for when stream starts.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_stream_started(|StreamStarted { title, .. }| {
    ///         owncast_send_chat(&format!("Stream {title} is starting!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_stream_started(&mut self, f: fn(&StreamStarted) -> ()) {
        self.on_stream_started_.push(f);
    }

    /// Creates an event hook for when stream stops.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_stream_stopped(|StreamStopped { stopped_at }| {
    ///         owncast_send_chat(&format!("Stream stopped at {stopped_at}!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_stream_stopped(&mut self, f: fn(&StreamStopped) -> ()) {
        self.on_stream_stopped_.push(f);
    }

    /// Creates an event hook for when the title of stream changes.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_stream_title_changed(|StreamTitleChange { from, to }| {
    ///         owncast_send_chat(&format!("Stream title changed from {from} to {to}."));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_stream_title_changed(&mut self, f: fn(&StreamTitleChange) -> ()) {
        self.on_stream_title_changed_.push(f);
    }

    /// Creates an event hook for when an SSE connection is made.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_sse_connect(|SSEConnectionEvent { connection_id, .. }| {
    ///         owncast_send_chat(&format!("Connected to {connection_id}."));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_sse_connect(&mut self, f: fn(&SSEConnectionEvent) -> ()) {
        self.on_sse_connect_.push(f);
    }

    /// Creates an event hook for when an SSE connection is ceased.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_sse_disconnect(|SSEConnectionEvent { connection_id, .. }| {
    ///         owncast_send_chat(&format!("Disconnected from {connection_id}."));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_sse_disconnect(&mut self, f: fn(&SSEConnectionEvent) -> ()) {
        self.on_sse_disconnect_.push(f);
    }

    /// Creates an event hook that is fired once a second.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_tick(|TickEvent { now }| {
    ///         owncast_send_chat(&format!("The time is now {now}"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_tick(&mut self, f: fn(&TickEvent) -> ()) {
        self.on_tick_.push(f);
    }

    /// Creates an event hook for when any incoming ActivityPub object is received.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse(|hash_map| {
    ///             TODO
    ///         owncast_send_chat(&format!("The time is now {now}"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse(&mut self, f: fn(&HashMap<String, String>) -> ()) {
        self.on_fediverse_.push(f);
    }

    /// Creates an event hook for when someone follows the stream through the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_follow(|FediverseEngagement { actor: FediverseActor { name, .. }, .. }| {
    ///         owncast_send_chat(&format!("{name} followed stream!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_follow(&mut self, f: fn(&FediverseEngagement) -> ()) {
        self.on_fediverse_follow_.push(f);
    }

    /// Creates an event hook for when someone likes a stream post made through the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_like(|FediverseTargetedEngagement { actor: FediverseActor { name, .. }, .. }| {
    ///         owncast_send_chat(&format!("{name} liked that stream went live!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_like(&mut self, f: fn(&FediverseTargetedEngagement) -> ()) {
        self.on_fediverse_like_.push(f);
    }

    /// Creates an event hook for when someone reposts a stream post made through the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_repost(|FediverseTargetedEngagement { actor: FediverseActor { name, .. }, .. }| {
    ///         owncast_send_chat(&format!("{name} reposted that stream went live!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_repost(&mut self, f: fn(&FediverseTargetedEngagement) -> ()) {
        self.on_fediverse_repost_.push(f);
    }

    /// Creates an event hook for when someone quotes a stream post made through the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_repost(|FediverseTargetedEngagement { actor: FediverseActor { name, .. }, .. }| {
    /// TODO include said quote
    ///         owncast_send_chat(&format!("{name} quoted that stream went live!"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_quote(&mut self, f: fn(&FediverseTargetedEngagement) -> ()) {
        self.on_fediverse_quote_.push(f);
    }

    /// Creates an event hook for when someone mentions stream's handle on the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_repost(|FediverseInboundPost { content_text, .. }| {
    ///         owncast_send_chat(&format!("Someone had this to say about stream: {content_text}"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_mention(&mut self, f: fn(&FediverseInboundPost) -> ()) {
        self.on_fediverse_mention_.push(f);
    }

    /// Creates an event hook for when someone replies to a stream post made through the ActivityPub protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_fediverse_repost(|FediverseInboundPost { content_text, .. }| {
    ///         owncast_send_chat(&format!("Someone had this to say in reply to stream: {content_text}"));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_fediverse_reply(&mut self, f: fn(&FediverseInboundPost) -> ()) {
        self.on_fediverse_reply_.push(f);
    }

    /// Creates a chat filter. Priority must be between 0 (inclusive) and 101 (exclusive). Defaults to 100.
    ///
    /// # Errors
    ///
    /// Errors if priority is greater than or equal to 101.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.filter_chat_message(None, |ChatMessage { body, .. }| {
    ///         if body.contains("bad word") {
    ///             FilterResult::Drop("No bad words allowed!".to_string())
    ///         } else {
    ///             FilterResult::Pass
    ///         }
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn filter_chat_message(&mut self, priority: Option<u8>, f: fn(&ChatMessage) -> FilterResult) -> Result<(), OutOfBounds> {
        // TODO if possible then put a compile-time restraint on priority.
        let priority = priority.unwrap_or(100);
        if priority >= 101 {
            Err(OutOfBounds("Filter priority must be between 0 (inclusive) and 101 (exclusive).".to_string()))
        } else {
            self.filter_chat_message_.push((priority, f));
            Ok(())
        }
    }

    /// Creates an event hook for when an http request is made to a given path with a given method. The callback function is a reference since multiple HTTP methods can run the same callback.
    ///
    /// # Errors
    ///
    /// Errors if the method and path combination are not unique.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_http_request(&[Method::GET], "/echo", &|IncomingHttpRequest { body, .. }| {
    ///         OutgoingHttpResponse {
    ///             status: Some(200),
    ///             headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
    ///             body: Some(body)
    ///         }
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_http_request(&mut self, method: &[Method], path: &str, f: &'a fn(&IncomingHttpRequest) -> OutgoingHttpResponse) -> Result<(), Duplicate> {
        for method in method {
            if let Some(_) = self.on_http_request_.insert((method.clone(), path.to_string()), f) {
                return Err(Duplicate(format!("An HTTP request handler already exists for {method} {path}.")));
            }
        }
        Ok(())
    }

    /// Creates an event hook for a custom event.
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Deserialize)]
    /// pub(crate) struct CustomEventPayload {
    ///     pub(crate) data: String
    /// }
    ///
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on("another-plugin.something", |CustomEventPayload { data }| {
    ///         owncast_send_chat(format!("Received {data}."));
    ///     });
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on<T: DeserializeOwned + 'static>(&mut self, event: &str, f: fn(&T) -> ()) {
        self.on_.push((event.to_string(), Box::new(move |payload| {
            f(&serde_json::from_str(payload)?);
            Ok(())
        })));
    }

    /// Registers commands. The callback function is a reference since aliased commands run the same callback.
    ///
    /// # Errors
    ///
    /// Errors if a duplicate command has been registered.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.commands("!", false, vec![
    ///         CommandBuilder::new("update", |ctx| {
    ///             ctx.reply("we've been live a while!");
    ///         })
    ///         .with_aliases(&["time", "livetime"])
    ///         .with_cooldown(1000)
    ///     ])?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn commands(&mut self, prefix: &str, case_sensitive: bool, command_builders: Vec<CommandBuilder>) -> Result<(), Duplicate> {
        for command_builder in command_builders {
            let command_data = command_builder.build(prefix.to_string(), case_sensitive);
            if let Some(CommandDefinition { command: Command { name, prefix, .. }, .. }) = self.commands_.insert(format!("{}{}", command_data.command.prefix, command_data.command.name), command_data) {
                return Err(Duplicate(format!("Command {prefix}{name} already exists.")));
            }
        }
        Ok(())
    }

    /// Registers content for when a specific tab is being viewed.
    ///
    /// # Errors
    ///
    /// Errors if a duplicate slug is registered.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_tab_content("store", |ContentRequest { user, .. }| {
    ///         format!("<p>Hello {user}!</p>")
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_tab_content(&mut self, tab: &str, f: fn(&ContentRequest) -> String) -> Result<(), Duplicate> {
        if let Some(_) = self.on_tab_content_.insert(tab.to_string(), f) {
            Err(Duplicate(format!("A tab content handler already exists for {tab}.")))
        } else {
            Ok(())
        }
    }

    /// Registers content for when a specific page is being viewed.
    ///
    /// # Errors
    ///
    /// Errors if a duplicate slug is registered.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_page_content("store", |ContentRequest { user, .. }| {
    ///         format!("<p>Hello {user}!</p>")
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_page_content(&mut self, page: &str, f: fn(&ContentRequest) -> String) -> Result<(), Duplicate> {
        if let Some(_) = self.on_page_content_.insert(page.to_string(), f) {
            Err(Duplicate(format!("A page content handler already exists for {page}.")))
        } else {
            Ok(())
        }
    }

    /// Registers a function for returning custom CSS.
    ///
    /// # Errors
    ///
    /// Errors if this function is called more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_page_styles(|| {
    ///         "* { font-size: 24pt; }".to_string()
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_page_styles(&mut self, f: fn() -> String) -> Result<(), Duplicate> {
        if self.on_page_styles_.is_some() {
            Err(Duplicate("Can only set on_page_styles once.".to_string()))
        } else {
            self.on_page_styles_ = Some(f);
            Ok(())
        }
    }

    /// Registers a function for returning custom client-side JS.
    ///
    /// # Errors
    ///
    /// Errors if this function is called more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_page_scripts(|| {
    ///         "alert('Welcome to stream!');".to_string()
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_page_scripts(&mut self, f: fn() -> String) -> Result<(), Duplicate> {
        if self.on_page_scripts_.is_some() {
            Err(Duplicate("Can only set on_page_scripts once.".to_string()))
        } else {
            self.on_page_scripts_ = Some(f);
            Ok(())
        }
    }

    /// Registers a function that implements an authentication gate.
    ///
    /// # Errors
    ///
    /// Errors if this function is called more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.on_auth_check(|AuthCheckRequest { user: User { display_name, .. } }| {
    ///         if display_name.as_str() == "Authorized user" {
    ///             AuthCheckResult::Ok()
    ///         } else {
    ///             AuthCheckResult::Deny("You are not authorized!".to_string())
    ///         }
    ///     })?;
    ///     Ok(plugin_builder)
    /// });
    /// ```
    pub fn on_auth_check(&mut self, f: fn(&AuthCheckRequest) -> AuthCheckResult) -> Result<(), Duplicate> {
        if self.on_auth_check_.is_some() {
            Err(Duplicate("Can only set on_page_scripts once.".to_string()))
        } else {
            self.on_auth_check_ = Some(f);
            Ok(())
        }
    }
}

impl<'a> Into<Plugin<'a>> for PluginBuilder<'a> {
    fn into(self) -> Plugin<'a> {
        let mut filter_chat_message_ = self.filter_chat_message_;
        filter_chat_message_.sort_by(|(a, _), (b, _)| {
            b.cmp(&a)
        });

        // Construct notifications.
        let mut notify = vec![];

        if !self.on_chat_message_.is_empty() { notify.push(Notify { event: CHAT_MESSAGE_RECEIVED.to_string() }); }
        if !self.on_chat_user_joined_.is_empty() { notify.push(Notify { event: CHAT_USER_JOINED.to_string() }); }
        if !self.on_chat_user_parted_.is_empty() { notify.push(Notify { event: CHAT_USER_PARTED.to_string() }); }
        if !self.on_chat_user_renamed_.is_empty() { notify.push(Notify { event: CHAT_USER_RENAMED.to_string() }); }
        if !self.on_message_moderated_.is_empty() { notify.push(Notify { event: CHAT_MESSAGE_MODERATED.to_string() }); }

        if !self.on_stream_started_.is_empty() { notify.push(Notify { event: STREAM_STARTED.to_string() }); }
        if !self.on_stream_stopped_.is_empty() { notify.push(Notify { event: STREAM_STOPPED.to_string() }); }
        if !self.on_stream_title_changed_.is_empty() { notify.push(Notify { event: STREAM_TITLE_CHANGED.to_string() }); }

        if !self.on_sse_connect_.is_empty() { notify.push(Notify { event: SSE_CONNECT.to_string() }); }
        if !self.on_sse_disconnect_.is_empty() { notify.push(Notify { event: SSE_DISCONNECT.to_string() }); }

        if !self.on_tick_.is_empty() { notify.push(Notify { event: TICK.to_string() }); }

        if !self.on_fediverse_.is_empty() { notify.push(Notify { event: FEDIVERSE_ACTIVITY.to_string() }); }
        if !self.on_fediverse_follow_.is_empty() { notify.push(Notify { event: FEDIVERSE_FOLLOW.to_string() }); }
        if !self.on_fediverse_like_.is_empty() { notify.push(Notify { event: FEDIVERSE_LIKE.to_string() }); }
        if !self.on_fediverse_repost_.is_empty() { notify.push(Notify { event: FEDIVERSE_REPOST.to_string() }); }
        if !self.on_fediverse_quote_.is_empty() { notify.push(Notify { event: FEDIVERSE_QUOTE.to_string() }); }
        if !self.on_fediverse_mention_.is_empty() { notify.push(Notify { event: FEDIVERSE_MENTION.to_string() }); }
        if !self.on_fediverse_reply_.is_empty() { notify.push(Notify { event: FEDIVERSE_REPLY.to_string() }); }

        // Construct filter.
        let mut filter = vec![];
        if let Some(&(priority, _)) = filter_chat_message_.first() {
            filter.push(Filter {
                event: "".to_string(),
                priority
            });
        }

        let subscriptions = Subscriptions { notify, filter };
        let commands: Vec<Command> = self.commands_.values().map(|CommandDefinition { command, .. }| command.clone()).collect();

        Plugin {
            manifest: Manifest::from((self.partial_manifest, subscriptions, commands)),

            on_chat_message: self.on_chat_message_,
            on_chat_user_joined: self.on_chat_user_joined_,
            on_chat_user_parted: self.on_chat_user_parted_,
            on_chat_user_renamed: self.on_chat_user_renamed_,
            on_message_moderated: self.on_message_moderated_,
            on_stream_started: self.on_stream_started_,
            on_stream_stopped: self.on_stream_stopped_,
            on_stream_title_changed: self.on_stream_title_changed_,
            on_sse_connect: self.on_sse_connect_,
            on_sse_disconnect: self.on_sse_disconnect_,
            on_tick: self.on_tick_,
            on_fediverse: self.on_fediverse_,
            on_fediverse_follow: self.on_fediverse_follow_,
            on_fediverse_like: self.on_fediverse_like_,
            on_fediverse_repost: self.on_fediverse_repost_,
            on_fediverse_quote: self.on_fediverse_quote_,
            on_fediverse_mention: self.on_fediverse_mention_,
            on_fediverse_reply: self.on_fediverse_reply_,
            on: self.on_,

            filter_chat_message: filter_chat_message_,

            on_http_request: self.on_http_request_,

            on_auth_check: self.on_auth_check_,

            on_tab_content: self.on_tab_content_,

            on_page_content: self.on_page_content_,

            on_page_styles: self.on_page_styles_,

            on_page_scripts: self.on_page_scripts_,

            commands: self.commands_
        }
    }
}