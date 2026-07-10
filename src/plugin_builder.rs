use std::collections::HashMap;
use crate::command_builder::CommandBuilder;
use crate::ctx::Ctx;
use crate::json_objects::command::Command;
use crate::method::Method;
use crate::json_objects::incoming_http_request::IncomingHttpRequest;
use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
use crate::plugin::Plugin;

pub(crate) type CommandData = (Box<dyn Fn(Ctx)>, Option<u128>, Option<Box<dyn Fn(Ctx)>>, Command);

pub struct PluginBuilder<'a> {
    on_chat_message_: Vec<Box<dyn Fn(&str)>>,
    on_http_request_: HashMap<(Method, String), Box<&'a dyn Fn(IncomingHttpRequest) -> OutgoingHttpResponse>>,
    on_: HashMap<String, Box<dyn Fn(&str)>>,

    commands_: HashMap<String, CommandData>
}

impl<'a> PluginBuilder<'a> {
    pub(crate) fn new() -> Self {
        Self {
            on_chat_message_: vec![],
            on_http_request_: HashMap::new(),
            on_: HashMap::new(),
            commands_: HashMap::new()
        }
    }

    /// Create an event hook for when a chat message is sent.
    pub fn on_chat_message<F: Fn(&str) -> () + 'static>(&mut self, f: F) {
        self.on_chat_message_.push(Box::new(f));
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
    pub fn commands(&mut self, prefix: &str, commands: Vec<CommandBuilder>) -> Result<(), String> {
        for command in commands {
            let command = command.build(prefix.to_string());

            if let Some(_) = self.commands_.insert(format!("{}{}", command.3.prefix, command.3.name), command) {
                return Err(format!("Command already exists."));
            }
        }
        Ok(())
    }
}

impl<'a> Into<Plugin<'a>> for PluginBuilder<'a> {
    fn into(self) -> Plugin<'a> {
        Plugin {
            on_chat_message: self.on_chat_message_,
            on_http_request: self.on_http_request_,
            on: self.on_,
            commands: self.commands_
        }
    }
}