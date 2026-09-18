use serde::Deserialize;
use crate::json_objects::attachment::Attachment;
use crate::json_objects::fediverse_actor::FediverseActor;

/// [`Envelope::FediverseMention`] and [`Envelope::FediverseReply`] event payloads.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseInboundPost {
    /// The fediverse actor in question.
    pub actor: FediverseActor,

    /// The unformatted content of the [`FediverseInboundPost`].
    pub content: String,

    /// The content of the [`FediverseInboundPost`] as plain text.
    pub content_text: String,

    /// The URL of the [`FediverseInboundPost`].
    pub url: String,

    /// When the [`FediverseInboundPost`] was posted.
    pub posted_at: String,

    /// What the [`FediverseInboundPost`] was replying to, if anything.
    pub in_reply_to: Option<String>,

    /// The attachments of the [`FediverseInboundPost`].
    pub attachments: Option<Vec<Attachment>>,

    /// The language of the [`FediverseInboundPost`].
    pub language: Option<String>,
}