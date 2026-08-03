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

impl FediversePayload {
    pub fn new(payload_type: FediversePayloadType, body: &str) -> Self {
        Self {
            payload_type,
            body: body.to_string(),
            image: None,
            link: None,
        }
    }

    pub fn with_image(mut self, image: &str) -> Self {
        self.image = Some(image.to_string());
        self
    }

    pub fn with_link(mut self, link: &str) -> Self {
        self.link = Some(link.to_string());
        self
    }
}