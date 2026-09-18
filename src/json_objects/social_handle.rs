use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object representing a link to a profile on an external website.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct SocialHandle {
    /// The human-readable name of the website the [`SocialHandle`] profile is on.
    pub platform: String, // TODO move to enum

    /// A URL to the [`SocialHandle`] profile.
    pub url: String,

    /// The icon used to represent the [`SocialHandle`] platform.
    pub icon: Option<String>
}