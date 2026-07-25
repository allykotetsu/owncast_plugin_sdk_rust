use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::chat_message::ChatMessage;

#[derive(ToBytes, Serialize)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum FilterResult {
    Pass,
    Modify { payload: ChatMessage },
    Drop { reason: String }
}

impl FilterResult {
    pub fn pass() -> Self {
        FilterResult::Pass
    }

    /// Construct a FilterResult::Modify from the original, unaltered chat message, and the new body.
    pub fn modify(chat_message: &ChatMessage, body: &str) -> Self {
        FilterResult::Modify {
            payload: ChatMessage {
                id: chat_message.id.clone(),
                user: chat_message.user.clone(),
                client_id: chat_message.client_id.clone(),
                body: body.to_string(),
                timestamp: chat_message.timestamp.clone(),
            }
        }
    }

    pub fn drop(reason: &str) -> Self {
        FilterResult::Drop {
            reason: reason.to_string()
        }
    }
}