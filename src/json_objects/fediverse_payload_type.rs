use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// A fediverse activity type.
#[derive(Serialize, ToBytes, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub enum FediversePayloadType {
    /// A fediverse activity for following an account on the fediverse.
    Follow,

    /// A fediverse activity for liking a post on the fediverse.
    Like,

    /// A fediverse activity for reposting a post on the fediverse.
    Repost,

    /// An arbitrary fediverse activity.
    #[serde(untagged)]
    Custom(String)
}