use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub enum FediversePayloadType {
    Follow,
    Like,
    Repost,
    #[serde(untagged)]
    Custom(String)
}