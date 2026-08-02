use serde::Deserialize;
use crate::json_objects::attachment::Attachment;
use crate::json_objects::fediverse_actor::FediverseActor;
use crate::json_objects::url::Url;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseQuote {
    pub actor: FediverseActor,
    pub target: Url,
    pub content: Option<String>,
    pub content_text: Option<String>,
    pub url: String,
    pub posted_at: Option<String>,
    pub in_reply_to: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub language: Option<String>
}