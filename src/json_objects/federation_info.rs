use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object containing information about the Owncast server's fediverse capabilities.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct FederationInfo {
    /// Whether federation is enabled or not.
    pub enabled: bool,

    /// The username being used on the fediverse.
    pub username: Option<String>,

    /// Whether the admin must approve fediverse followers.
    pub is_private: Option<bool>
}