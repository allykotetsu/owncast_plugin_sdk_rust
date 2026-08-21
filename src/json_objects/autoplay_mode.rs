use extism_pdk::{FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
#[encoding(Json)]
pub enum AutoplayMode {
    Off,
    Always,
    SoundOnly
}