use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::chat_message::ChatMessage;

/// The result of a filter on a chat message.
#[derive(ToBytes, Serialize, Clone, Debug)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum FilterResult {
    /// The filter lets the message pass unchanged.
    Pass,

    /// The filter modifies the message.
    Modify {
        /// The chat message data to replace the one being filtered.
        payload: ChatMessage
    },

    /// The filter prevents the message from being sent.
    Drop {
        /// The reason why a message was filtered out.
        reason: String
    }
}

impl FilterResult {
    pub fn pass() -> Self {
        FilterResult::Pass
    }

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