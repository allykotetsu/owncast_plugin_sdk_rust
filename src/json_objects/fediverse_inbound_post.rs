use serde::Deserialize;
use crate::json_objects::attachment::Attachment;
use crate::json_objects::fediverse_actor::FediverseActor;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FediverseInboundPost {
    actor: FediverseActor,
    content: String,
    content_text: String,
    url: String,
    posted_at: String,
    in_reply_to: Option<String>,
    attachments: Option<Vec<Attachment>>,
    language: Option<String>,
}