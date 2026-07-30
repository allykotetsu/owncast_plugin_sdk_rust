use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct SocialHandle {
    pub platform: String,
    pub url: String,
    pub icon: Option<String>
}