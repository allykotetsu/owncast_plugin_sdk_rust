use extism_pdk::{FromBytes, Json};
use serde::{Deserialize, Serialize};

/// An object representing a user viewing an Owncast stream. Also a payload for [`Envelope::ChatUserJoined`] and [`Envelope::ChatUserParted`].
#[derive(Serialize, Deserialize, Clone, FromBytes, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct User {
    /// The unique shortid of the [`User`].
    pub id: String,

    /// The display name of the [`User`].
    pub display_name: String,

    /// The username color of the [`User`].
    pub display_color: u16,

    /// The previous display names the [`User`] has used.
    pub previous_names: Option<Vec<String>>,

    /// When the [`User`] was created.
    pub created_at: Option<String>,

    /// When the [`User`] was disabled, if banned.
    pub disabled_at: Option<String>,

    /// The scopes of the [`User`], such as "MODERATOR".
    pub scopes: Option<Vec<String>>,

    /// Whether the [`User`] is a bot.
    pub is_bot: Option<bool>,

    /// Whether the [`User`] is an admin.
    pub is_authenticated: Option<bool>
}