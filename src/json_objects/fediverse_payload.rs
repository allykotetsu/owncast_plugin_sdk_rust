use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::fediverse_payload_type::FediversePayloadType;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct FediversePayload {
    #[serde(rename = "type")]
    pub payload_type: FediversePayloadType,
    pub body: String,
    pub image: Option<String>,
    pub link: Option<String>
}