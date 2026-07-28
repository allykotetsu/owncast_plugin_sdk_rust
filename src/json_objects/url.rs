use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct Url {
    pub url: String
}