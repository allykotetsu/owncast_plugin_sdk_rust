use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object containing information about the stream data Owncast is receiving.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamBroadcaster {
    /// The IP address of the source of the Owncast stream.
    pub remote_addr: Option<String>,

    /// The codecs of the incoming stream.
    pub codecs: Option<Vec<String>>,

    /// The resolution of the incoming stream.
    pub resolution: Option<String>,

    /// The framerate of the incoming stream.
    pub framerate: Option<i64>,

    /// The bitrates of the incoming stream.
    pub bitrates: Option<Vec<i64>>
}