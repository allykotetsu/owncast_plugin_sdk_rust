use std::collections::HashMap;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_data::CommandData;
use crate::json_objects::filter_result::FilterResult;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::plugin::Plugin;


pub struct PluginBuilder<'a> {
    // Events
    on_chat_message_: Vec<Box<dyn Fn(&str)>>,
    on_: HashMap<String, Box<dyn Fn(&str)>>,

    // Filter
    filter_chat_message_: Vec<(u8, Box<dyn Fn(&str) -> FilterResult>)>,

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
            on_: HashMap::new(),
            commands_: HashMap::new()
        }
    }

    // TODO replace with generic "on_event" function? Or maybe do that internally? idk
    /// Create an event hook for when a chat message is sent.
    pub fn on_chat_message<F: Fn(&str) -> () + 'static>(&mut self, f: F) {
        self.on_chat_message_.push(Box::new(f));
    }

    /// Create a chat filter. Errors if priority is out of bounds.
    pub fn filter_chat_message<F: Fn(&str) -> FilterResult + 'static>(&mut self, priority: Option<u8>, f: F) -> Result<(), String> {
        let priority = priority.unwrap_or(100);
        if priority < 0 || priority >= 101 {
            Err("Filter priority must be between 0 (inclusive) and 101 (exclusive).".to_string())
        } else {
            self.filter_chat_message_.push((priority, Box::new(f)));
            Ok(())
        }
    }

    /// Create an event hook for when an http request is made to a given path with a given method. Errors if the method and path combination are not unique.
    pub fn on_http_request<F: Fn(IncomingHttpRequest) -> OutgoingHttpResponse + 'static>(&mut self, method: &[Method], path: &str, f: &'a F) -> Result<(), String> {
        for method in method {
            if let Some(_) = self.on_http_request_.insert((method.clone(), path.to_string()), Box::new(f)) {
                return Err(format!("An HTTP request handler already exists for {method} {path}."));
            }
        }

        Ok(())
    }

    // TODO replace with a macro that can handle any type (and not just &str)
    /// Create an event hook for a custom event. Errors if an event with that name is already being listened for.
    pub fn on<F: Fn(&str) -> () + 'static>(&mut self, event: &str, f: F) -> Result<(), String> {
        match self.on_.insert(event.to_string(), Box::new(f)) {
            Some(_) => Err(format!("Event {event} already exists.")),
            None => Ok(())
        }
    }

    /// Register commands. Errors if duplicate commands are registered.
    pub fn commands(&mut self, prefix: &str, commands: Vec<CommandBuilder<'a>>) -> Result<(), String> {
        for command in commands {
            let command = command.build(prefix.to_string());
            if let Some(_) = self.commands_.insert(format!("{}{}", command.command.prefix, command.command.name), command) {
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