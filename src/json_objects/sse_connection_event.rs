use serde::Deserialize;
use crate::json_objects::user::User;

/// [`Envelope::SseConnect`] and [`Envelope::SseDisconnect`] event payloads.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SseConnectionEvent {
    /// The named SSE channel.
    pub channel: String,

    /// The unique ID of the connection instance.
    pub connection_id: u64,

    /// The [`User`] dis/connecting to SSE.
    pub user: Option<User>
}