use serde::Deserialize;
use crate::json_objects::user::User;

/// [`Envelope::ChatMessageModerated`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageModeration {
    /// The unique ID of the [`ChatMessage`] being moderated.
    pub message_id: String,

    /// Whether the [`ChatMessage`] being moderated is now visible.
    pub visible: bool,

    /// The [`User`] that moderated the [`ChatMessage`].
    pub moderator: Option<User>
}