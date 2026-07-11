use serde::Serialize;
use crate::json_objects::chat_message::ChatMessage;
// TODO make sure it serializes correctly

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterResult {
    Pass,
    Modify(ChatMessage),
    Drop(String)
}