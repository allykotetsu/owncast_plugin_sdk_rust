use crate::json_objects::user::User;
use crate::json_objects::chat_message::ChatMessage;
use crate::{owncast, run};

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub msg: ChatMessage,
    pub user: Option<User>,
    pub command: String,
    pub invoked_as: String,
    pub args: Vec<String>,
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