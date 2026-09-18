use extism_pdk::{FromBytes, Json};
use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

/// An object representing a message sent in chat.
#[derive(Serialize, Deserialize, Clone, FromBytes, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatMessage {
    /// The unique ID of the [`ChatMessage`].
    pub id: String,

    /// The user that sent the [`ChatMessage`].
    pub user: Option<User>,

    /// The ID of the client that sent the [`ChatMessage`].
    pub client_id: Option<i64>,

    /// The body of the [`ChatMessage`].
    pub body: String,

    /// The time that the [`ChatMessage`] was sent.
    pub timestamp: String
}

impl TryFrom<&ChatMessage> for i64 {
    type Error = ();

    fn try_from(ChatMessage { client_id, .. } : &ChatMessage) -> Result<Self, Self::Error> {
        match client_id {
            None => Err(()),
            Some(client_id) => Ok(*client_id)
        }
    }
}