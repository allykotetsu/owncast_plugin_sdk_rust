use serde::Deserialize;
use crate::json_objects::fediverse_actor::FediverseActor;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FediverseTargetedEngagement {
    actor: FediverseActor,
    target: String,
}