use extism_pdk::{FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};

/// The autoplay mode for stream.
#[derive(Deserialize, Serialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
#[encoding(Json)]
pub enum AutoplayMode {
    /// Do not autoplay stream.
    Off,

    /// Autoplay stream when the page is loaded.
    Always,

    /// Autoplay the stream with sound-only when the page is loaded.
    SoundOnly
}