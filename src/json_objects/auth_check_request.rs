use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::user::User;

/// A authorization request for when a plugin auth-gates Owncast.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct AuthCheckRequest {
    /// The [`User`] making the [`AuthCheckRequest`]
    pub user: User
}