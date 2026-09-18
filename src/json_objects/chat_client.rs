use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object representing an individual client watching the Owncast stream through the homepage or an embed.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatClient {
    /// The unique numeral ID of the [`ChatClient`].
    pub id: u64,

    /// The ID of the [`User`] running the [`ChatClient`].
    pub user_id: Option<String>,

    /// The display name of the [`User`] of the [`ChatClient`].
    pub display_name: Option<String>,

    /// The ISO-8601 timestamp for when the [`ChatClient`] connected.
    pub connected_at: Option<String>,

    /// The browser, app, or other type of agent managing the [`ChatClient`].
    pub user_agent: Option<String>,

    /// The IP address of the [`ChatClient`].
    pub ip_address: Option<String>,

    /// How many [`ChatMessage`]s the [`User`] running the [`ChatClient`] has sent.
    pub message_count: i64,
}