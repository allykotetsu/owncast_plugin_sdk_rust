use serde::Deserialize;
use crate::json_objects::fediverse_actor::FediverseActor;

/// [`Envelope::FediverseLike`] and [`Envelope::FediverseRepost`] event payloads.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseTargetedEngagement {
    /// The fediverse actor in question.
    pub actor: FediverseActor,

    /// What they are engaging with.
    pub target: String,
}