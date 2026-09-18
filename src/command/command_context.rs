use crate::json_objects::user::User;
use crate::json_objects::chat_message::ChatMessage;
use crate::{owncast, run};

/// An object representing relevant information to a chat command being used.
#[derive(Debug, Clone)]
pub struct CommandContext {
    /// The [`ChatMessage`] that fired the chat command event.
    pub msg: ChatMessage,

    /// The [`User`] that used the chat command.
    pub user: Option<User>,

    /// The alias the chat command was invoked as.
    pub invoked_as: String,

    /// The space-separated arguments to the chat command.
    pub args: Vec<String>,

    /// The un-split argument string.
    pub arg_string: String
}

impl CommandContext {
    pub fn reply(&self, text: &str) {
        run!(owncast::chat::send(text));
    }

    pub fn reply_privately(&self, text: &str) {
        if let Ok(false) = owncast::chat::reply_to(&self.msg, text) {
            run!(owncast::chat::send(text));
        }
    }
}