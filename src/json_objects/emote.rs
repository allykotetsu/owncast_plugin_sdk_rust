use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object representing a custom emoji used in stream chat.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct Emote {
    /// THe unique name of the [`Emote`].
    pub name: String,

    /// The URL of the [`Emote`]'s image file.
    pub url: String
}