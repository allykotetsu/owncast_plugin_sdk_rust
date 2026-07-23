use crate::errors::Forbidden;
use crate::json_objects::user::User;
use crate::imports::owncast_send_chat;
use crate::imports::owncast_send_chat_reply;
use crate::json_objects::chat_message::ChatMessage;
use crate::output_json::OutputJson;

pub(crate) struct CommandContext {
    pub(crate) msg: ChatMessage,
    pub(crate) user: Option<User>,
    pub(crate) command: String,
    pub(crate) invoked_as: String,
    pub(crate) args: Vec<String>,
    pub(crate) arg_string: String
}

impl CommandContext {
    pub fn reply(&self, text: &str) -> Result<(), Forbidden> {
        Ok(owncast_send_chat(text)?)
    }

    pub fn reply_privately(&self, text: &str) -> Result<(), Forbidden> {
        if !owncast_send_chat_reply(OutputJson(self.msg.clone()), text)? {
            owncast_send_chat(text)?;
        }
        Ok(())
    }
}