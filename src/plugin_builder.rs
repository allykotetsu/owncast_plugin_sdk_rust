use std::collections::HashMap;
use std::fs::read_to_string;
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_definition::CommandDefinition;
use crate::json_objects::auth_check_request::AuthCheckRequest;
use crate::json_objects::auth_check_result::AuthCheckResult;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::chat_message_moderation::ChatMessageModeration;
use crate::json_objects::chat_user_rename::ChatUserRename;
use crate::json_objects::command::Command;
use crate::json_objects::content_request::ContentRequest;
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
    on_tab_content_: Option<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Content
    on_page_content_: Option<Box<dyn Fn(&ContentRequest) -> String>>,

    // Page Styles
    on_page_styles_: Vec<Box<dyn Fn() -> String>>,

    // Page Scripts
    on_page_scripts_: Vec<Box<dyn Fn() -> String>>,

    // Commands
    commands_: HashMap<String, CommandDefinition<'a>>
}

impl<'a> PluginBuilder<'a> {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            partial_manifest: serde_json::from_str(&read_to_string("./plugin.manifest.json")?)?,

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

            on_tab_content_: None,

            on_page_content_: None,

            on_page_styles_: vec![],

            on_page_scripts_: vec![],

            commands_: HashMap::new()
        })
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
    pub fn on_chat_message<F: Fn(&ChatMessage) -> () + 'static>(&mut self, f: F) {
        self.on_chat_message_.push(Box::new(f));
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
    pub fn on_chat_user_joined<F: Fn(&User) -> () + 'static>(&mut self, f: F) {
        self.on_chat_user_joined_.push(Box::new(f));
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
    pub fn on_chat_user_parted<F: Fn(&User) -> () + 'static>(&mut self, f: F) {
        self.on_chat_user_parted_.push(Box::new(f));
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
    pub fn on_chat_user_renamed<F: Fn(&ChatUserRename) -> () + 'static>(&mut self, f: F) {
        self.on_chat_user_renamed_.push(Box::new(f));
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
    pub fn on_message_moderated<F: Fn(&ChatMessageModeration) -> () + 'static>(&mut self, f: F) {
        self.on_message_moderated_.push(Box::new(f));
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
    pub fn on_stream_started<F: Fn(&StreamStarted) -> () + 'static>(&mut self, f: F) {
        self.on_stream_started_.push(Box::new(f));
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
    pub fn on_stream_stopped<F: Fn(&StreamStopped) -> () + 'static>(&mut self, f: F) {
        self.on_stream_stopped_.push(Box::new(f));
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
    pub fn on_stream_title_changed<F: Fn(&StreamTitleChange) -> () + 'static>(&mut self, f: F) {
        self.on_stream_title_changed_.push(Box::new(f));
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
    pub fn on_sse_connect<F: Fn(&SSEConnectionEvent) -> () + 'static>(&mut self, f: F) {
        self.on_sse_connect_.push(Box::new(f));
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
    pub fn on_sse_disconnect<F: Fn(&SSEConnectionEvent) -> () + 'static>(&mut self, f: F) {
        self.on_sse_disconnect_.push(Box::new(f));
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
    pub fn on_tick<F: Fn(&TickEvent) -> () + 'static>(&mut self, f: F) {
        self.on_tick_.push(Box::new(f));
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
    pub fn on_fediverse<F: Fn(&HashMap<String, String>) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_.push(Box::new(f));
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
    pub fn on_fediverse_follow<F: Fn(&FediverseEngagement) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_follow_.push(Box::new(f));
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
    pub fn on_fediverse_like<F: Fn(&FediverseTargetedEngagement) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_like_.push(Box::new(f));
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
    pub fn on_fediverse_repost<F: Fn(&FediverseTargetedEngagement) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_repost_.push(Box::new(f));
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
    pub fn on_fediverse_quote<F: Fn(&FediverseTargetedEngagement) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_quote_.push(Box::new(f));
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
    pub fn on_fediverse_mention<F: Fn(&FediverseInboundPost) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_mention_.push(Box::new(f));
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
    pub fn on_fediverse_reply<F: Fn(&FediverseInboundPost) -> () + 'static>(&mut self, f: F) {
        self.on_fediverse_reply_.push(Box::new(f));
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
    /// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    ///     plugin_builder.filter_chat_message(None, |ChatMessage { body, .. }| {
    ///         if body.contains("bad word") {
    ///             FilterResult::Drop("No bad words allowed!".to_string())
    ///         } else {
    ///             FilterResult::Pass
    ///         }
    ///     })?;
    /// });
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

    /// Creates an event hook for when an http request is made to a given path with a given method. The callback function is a reference since multiple HTTP methods can run the same callback.
    ///
    /// # Panics
    ///
    /// Panics if the method and path combination are not unique.
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
    /// });
    /// ```
    pub fn on_http_request<F: Fn(&IncomingHttpRequest) -> OutgoingHttpResponse + 'static>(&mut self, method: &[Method], path: &str, f: &'a F) -> Result<(), String> {
        for method in method {
            if let Some(_) = self.on_http_request_.insert((method.clone(), path.to_string()), Box::new(f)) {
                return Err(format!("An HTTP request handler already exists for {method} {path}."));
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
    /// });
    /// ```
    pub fn on<T: DeserializeOwned, F: Fn(&T) -> () + 'static>(&mut self, event: &str, f: F) {
        self.on_.push((event.to_string(), Box::new(move |payload| {
            f(&serde_json::from_str(payload)?);
            Ok(())
        })));
    }

    /// Registers commands. The callback function is a reference since aliased commands run the same callback.
    ///
    /// # Panics
    ///
    /// Panics if a duplicate command has been registered.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.commands("!", false, vec![
    ///     CommandBuilder::new("update", &|ctx| {
    ///         ctx.reply("we've been live a while!");
    ///     })
    ///     .with_aliases(&["time", "livetime"])
    ///     .with_cooldown(1000)
    /// ])?;
    /// ```
    pub fn commands(&mut self, prefix: &str, case_sensitive: bool, command_builders: Vec<CommandBuilder<'a>>) -> Result<(), String> {
        for command_builder in command_builders {
            let command_data = command_builder.build(prefix.to_string(), case_sensitive);
            if let Some(CommandDefinition { command: Command { name, prefix, .. }, .. }) = self.commands_.insert(format!("{}{}", command_data.command.prefix, command_data.command.name), command_data) {
                return Err(format!("Command {prefix}{name} already exists."));
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

        // TODO
        // Construct notifications.
        let mut notify = vec![];
        if !self.on_chat_message_.is_empty() {
            notify.push(Notify { event: "".to_string() });
        }

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

            commands: self.commands_,
        }
    }
}