use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct FederationInfo {
    pub enabled: bool,
    pub username: Option<String>,
    pub is_private: Option<bool>
}