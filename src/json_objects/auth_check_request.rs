use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct AuthCheckRequest {
    pub(crate) user: User
}