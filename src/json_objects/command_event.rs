use serde::Deserialize;
use crate::json_objects::chat_message::ChatMessage;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandEvent {
    pub(crate) message: ChatMessage,
    pub(crate) command: String,
    pub(crate) invoked_as: String,
    pub(crate) args: Vec<String>,
    pub(crate) arg_string: String
}