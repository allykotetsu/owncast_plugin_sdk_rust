use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object representing a URL.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct Url {
    /// The text location of the [`Url`].
    pub url: String
}