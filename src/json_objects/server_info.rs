use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ServerInfo {
    pub name: Option<String>,
    pub url: Option<String>,
    pub summary: Option<String>,
    pub welcome_message: Option<String>,
    pub version: Option<String>
}