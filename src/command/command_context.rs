use crate::json_objects::user::User;
use crate::json_objects::chat_message::ChatMessage;
use crate::imports::funcs::owncast_send_chat;
use crate::imports::funcs::owncast_send_chat_to;

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
        owncast_send_chat(text)
    }

    pub fn reply_privately(&self, text: &str) {
        if let Some(client_id) = self.msg.client_id {
            owncast_send_chat_to(client_id, text)
        } else {
            owncast_send_chat(text)
        }
    }
}