use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::user::User;

/// An object representing a request for a user to access a resource with a plugin-unique slug,
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ContentRequest {
    /// The plugin-unique, human-readable text ID for the [`ContentRequest`].
    pub slug: String,

    /// The [`User`] making the [`ContentRequest`].
    pub user: Option<User>
}