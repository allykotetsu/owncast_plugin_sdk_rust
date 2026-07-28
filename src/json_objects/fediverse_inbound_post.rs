use serde::Deserialize;
use crate::json_objects::attachment::Attachment;
use crate::json_objects::fediverse_actor::FediverseActor;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FediverseInboundPost {
    pub actor: FediverseActor,
    pub content: String,
    pub content_text: String,
    pub url: String,
    pub posted_at: String,
    pub in_reply_to: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub language: Option<String>,
}