use std::collections::HashMap;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_data::CommandData;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::plugin::Plugin;

pub struct PluginBuilder<'a> {
    // Events
    on_chat_message_: Vec<Box<dyn Fn(ChatMessage)>>,
    on_: Vec<(String, Box<dyn Fn(&str)>)>,

    // Filter
    filter_chat_message_: Vec<(u8, Box<dyn Fn(ChatMessage) -> FilterResult>)>,

    // HTTP
    on_http_request_: HashMap<(Method, String), Box<&'a dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>,

    // Commands
    commands_: HashMap<String, CommandData<'a>>
}

impl<'a> PluginBuilder<'a> {
    pub(crate) fn new() -> Self {
        Self {
            on_chat_message_: vec![],
            filter_chat_message_: vec![],
            on_http_request_: HashMap::new(),
            on_: vec![],
            commands_: HashMap::new()
        }
    }

    // TODO replace with generic "on_event" function? Or maybe do that internally? idk
    /// Creates an event hook for when a chat message is sent.
    ///
    /// # Examples
    ///
    /// ```
    /// plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
    ///     owncast_send_chat(format!("echo ${body}").as_str());
    /// });
    /// ```
    pub fn on_chat_message<F: Fn(ChatMessage) -> () + 'static>(&mut self, f: F) {
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
    /// plugin_builder.filter_chat_message(None, |ChatMessage { body, .. }| {
    ///     if body.contains("bad word") {
    ///         FilterResult::Drop("No bad words allowed!".to_string())
    ///     } else {
    ///         FilterResult::Pass
    ///     }
    /// })?;
    /// ```
    pub fn filter_chat_message<F: Fn(ChatMessage) -> FilterResult + 'static>(&mut self, priority: Option<u8>, f: F) -> Result<(), String> {
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
    /// plugin_builder.on_http_request(&[Method::GET], "/echo", &|IncomingHttpRequest { body, .. }: IncomingHttpRequest| {
    ///     OutgoingHttpResponse {
    ///         status: Some(200),
    ///         headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
    ///         body: Some(body)
    ///     }
    /// })?;
    /// ```
    pub fn on_http_request<F: Fn(IncomingHttpRequest) -> OutgoingHttpResponse + 'static>(&mut self, method: &[Method], path: &str, f: &'a F) -> Result<(), String> {
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
    /// plugin_builder.on("another-plugin.something", |payload| {
    ///     // idk
    /// });
    /// ```
    pub fn on<F: Fn(&str) -> () + 'static>(&mut self, event: &str, f: F) {
        self.on_.push((event.to_string(), Box::new(f)));
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
    ///     CommandBuilder::new("update", &|ctx: Ctx| {
    ///         ctx.reply("we've been live a while!");
    ///     })
    ///     .with_aliases(&["time", "livetime"])
    ///     .with_cooldown(1000)
    /// ])?;
    /// ```
    pub fn commands(&mut self, prefix: &str, command_builders: Vec<CommandBuilder<'a>>) -> Result<(), String> {
        for command_builder in command_builders {
            let command_data = command_builder.build(prefix.to_string());

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
            filter_chat_message: filter_chat_message_,
            on_http_request: self.on_http_request_,
            on: self.on_,
            commands: self.commands_
        }
    }
}