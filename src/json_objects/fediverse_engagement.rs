use serde::Deserialize;
use crate::json_objects::fediverse_actor::FediverseActor;

/// [`Envelope::FediverseFollow`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseEngagement {
    /// The fediverse actor in question.
    pub actor: FediverseActor,
}