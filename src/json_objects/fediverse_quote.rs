use serde::Deserialize;
use crate::json_objects::attachment::Attachment;
use crate::json_objects::fediverse_actor::FediverseActor;
use crate::json_objects::url::Url;

/// [`Envelope::FediverseQuote`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseQuote {
    /// The fediverse actor in question.
    pub actor: FediverseActor,

    /// What the [`FediverseQuote`] was quoting.
    pub target: Url,

    /// The unformatted caption of the [`FediverseQuote`].
    pub content: Option<String>,

    /// The caption of the [`FediverseQuote`] as plain text.
    pub content_text: Option<String>,

    /// The URL of the [`FediverseQuote`].
    pub url: String,

    /// When the [`FediverseQuote`] was posted.
    pub posted_at: Option<String>,

    /// What the [`FediverseQuote`] was replying to, if anything.
    pub in_reply_to: Option<String>,

    /// The attachments of the [`FediverseQuote`].
    pub attachments: Option<Vec<Attachment>>,

    /// The language of the [`FediverseQuote`].
    pub language: Option<String>
}