use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamBroadcaster {
    pub remote_addr: Option<String>,
    pub codecs: Option<Vec<String>>,
    pub resolution: Option<String>,
    pub framerate: Option<i64>,
    pub bitrates: Option<Vec<i64>>
}