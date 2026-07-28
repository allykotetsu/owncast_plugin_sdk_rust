use crate::json_objects::user::User;
use crate::json_objects::chat_message::ChatMessage;
use crate::owncast;
use crate::run;

pub struct CommandContext {
    pub(crate) msg: ChatMessage,
    pub(crate) user: Option<User>,
    pub(crate) command: String,
    pub(crate) invoked_as: String,
    pub(crate) args: Vec<String>,
    pub(crate) arg_string: String
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