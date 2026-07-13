use std::collections::HashMap;
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_data::CommandDefinition;
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::content_request::ContentRequest;
use crate::json_objects::fediverse_engagement::FediverseEngagement;
use crate::json_objects::fediverse_inbound_post::FediverseInboundPost;
use crate::json_objects::fediverse_targeted_engagement::FediverseTargetedEngagement;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::json_objects::sse_connection_event::SSEConnectionEvent;
use crate::json_objects::stream_started::StreamStarted;
use crate::json_objects::stream_stopped::StreamStopped;
use crate::json_objects::stream_title_change::StreamTitleChange;
use crate::json_objects::tick_event::TickEvent;
use crate::json_objects::user::User;
use crate::plugin::Plugin;

pub struct PluginBuilder<'a> {
    // Events
    on_chat_message_: Vec<Box<dyn Fn(&ChatMessage)>>,
    on_chat_user_joined_: Vec<Box<dyn Fn(&User)>>,
    on_chat_user_parted_: Vec<Box<dyn Fn(&User)>>,
    on_chat_user_renamed_: Vec<Box<dyn Fn(&ChatUserRename)>>,
    on_message_moderated_: Vec<Box<dyn Fn(&ChatMessageModeration)>>,

    on_stream_started_: Vec<Box<dyn Fn(&StreamStarted)>>,
    on_stream_stopped_: Vec<Box<dyn Fn(&StreamStopped)>>,
    on_stream_title_changed_: Vec<Box<dyn Fn(&StreamTitleChange)>>,

    on_sse_connect_: Vec<Box<dyn Fn(&SSEConnectionEvent)>>,
    on_sse_disconnect_: Vec<Box<dyn Fn(&SSEConnectionEvent)>>,

    on_tick_: Vec<Box<dyn Fn(&TickEvent)>>,

    on_fediverse_: Vec<Box<dyn Fn(&HashMap<String, String>)>>,
    on_fediverse_follow_: Vec<Box<dyn Fn(&FediverseEngagement)>>,
    on_fediverse_like_: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    on_fediverse_repost_: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    on_fediverse_quote_: Vec<Box<dyn Fn(&FediverseTargetedEngagement)>>,
    on_fediverse_mention_: Vec<Box<dyn Fn(&FediverseInboundPost)>>,
    on_fediverse_reply_: Vec<Box<dyn Fn(&FediverseInboundPost)>>,

    on_: Vec<(String, Box<dyn Fn(&str) -> Result<(), Error>>)>,

    // Filter
    filter_chat_message_: Vec<(u8, Box<dyn Fn(&ChatMessage) -> FilterResult>)>,

    // HTTP
    on_http_request_: HashMap<(Method, String), Box<&'a dyn Fn(&IncomingHttpRequest) -> OutgoingHttpResponse>>,

    // Auth Check
    on_auth_check_: Vec<Box<dyn Fn(&AuthCheckRequest) -> AuthCheckResult>>,

    // Tab Content
    on_tab_content_: Vec<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Content
    on_page_content_: Vec<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Styles
    on_page_styles_: Vec<Box<dyn Fn() -> String>>,

    // Page Scripts
    on_page_scripts_: Vec<Box<dyn Fn() -> String>>,

    // Commands
    commands_: HashMap<String, CommandDefinition<'a>>
}

impl<'a> PluginBuilder<'a> {
    pub(crate) fn new() -> Self {
        Self {
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

            on_auth_check_: vec![],

            on_tab_content_: vec![],

            on_page_content_: vec![],

            on_page_styles_: vec![],

            on_page_scripts_: vec![],

            commands_: HashMap::new()
        }
    }

    // TODO replace with generic "on_event" function? Or maybe do that internally? idk
    /// Creates an event hook for when a chat message is sent.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.on_chat_message(|&ChatMessage { body, .. }| {
    ///     owncast_send_chat(format!("echo ${body}").as_str());
    /// });
    /// ```
    pub fn on_chat_message<F: Fn(&ChatMessage) -> () + 'static>(&mut self, f: F) {
        self.on_chat_message_.push(Box::new(f));
    }

    /// Creates a chat filter. Priority must be between 0 (inclusive) and 101 (exclusive). Defaults to 100.
    ///
    /// # Panics
    ///
    /// Panics if priority is greater than or equal to 101.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.filter_chat_message(None, |&ChatMessage { body, .. }| {
    ///     if body.contains("bad word") {
    ///         FilterResult::Drop("No bad words allowed!".to_string())
    ///     } else {
    ///         FilterResult::Pass
    ///     }
    /// })?;
    /// ```
    pub fn filter_chat_message<F: Fn(&ChatMessage) -> FilterResult + 'static>(&mut self, priority: Option<u8>, f: F) -> Result<(), String> {
        // TODO if possible then put a compile-time restraint on priority.
        let priority = priority.unwrap_or(100);
        if priority >= 101 {
            Err("Filter priority must be between 0 (inclusive) and 101 (exclusive).".to_string())
        } else {
            self.filter_chat_message_.push((priority, Box::new(f)));
            Ok(())
        }
    }

    /// Creates an event hook for when an http request is made to a given path with a given method.
    ///
    /// # Panics
    ///
    /// Panics if the method and path combination are not unique.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.on_http_request(&[Method::GET], "/echo", &|&IncomingHttpRequest { body, .. }: IncomingHttpRequest| {
    ///     OutgoingHttpResponse {
    ///         status: Some(200),
    ///         headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
    ///         body: Some(body)
    ///     }
    /// })?;
    /// ```
    pub fn on_http_request<F: Fn(&IncomingHttpRequest) -> OutgoingHttpResponse + 'static>(&mut self, method: &[Method], path: &str, f: &'a F) -> Result<(), String> {
        for method in method {
            if let Some(_) = self.on_http_request_.insert((method.clone(), path.to_string()), Box::new(f)) {
                return Err(format!("An HTTP request handler already exists for {method} {path}."));
            }
        }

        Ok(())
    }

    // TODO replace with a macro that can handle any type (and not just &str)?
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
    /// plugin_builder.on("another-plugin.something", |CustomEventPayload { data }: &CustomEventPayload| {
    ///     owncast_send_chat(format!("Received {data}."));
    /// });
    /// ```
    pub fn on<T: DeserializeOwned, F: Fn(&T) -> () + 'static>(&mut self, event: &str, f: F) {
        self.on_.push((event.to_string(), Box::new(move |payload| {
            f(&serde_json::from_str(payload)?);
            Ok(())
        })));
    }

    /// Registers commands.
    ///
    /// # Panics
    ///
    /// Panics if a duplicate command has been registered.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.commands("!", vec![
    ///     CommandBuilder::new("update", &|ctx: &Ctx| {
    ///         ctx.reply("we've been live a while!");
    ///     })
    ///     .with_aliases(&["time", "livetime"])
    ///     .with_cooldown(1000)
    /// ])?;
    /// ```
    pub fn commands(&mut self, prefix: &str, case_sensitive: bool, command_builders: Vec<CommandBuilder<'a>>) -> Result<(), String> {
        for command_builder in command_builders {
            let command_data = command_builder.build(prefix.to_string(), case_sensitive);

            // TODO iterate through aliases and insert a command for each alias.

            if let Some(_) = self.commands_.insert(format!("{}{}", command_data.command.prefix, command_data.command.name), command_data) {
                return Err(format!("Command already exists."));
            }
        }
        Ok(())
    }
}

impl<'a> Into<Plugin<'a>> for PluginBuilder<'a> {
    fn into(self) -> Plugin<'a> {
        let mut filter_chat_message_ = self.filter_chat_message_;
        filter_chat_message_.sort_by(|(a, _), (b, _)| {
            b.cmp(&a)
        });

        Plugin {
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

            commands: self.commands_,
        }
    }
}