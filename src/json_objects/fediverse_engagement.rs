use serde::Deserialize;
use crate::json_objects::fediverse_actor::FediverseActor;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseEngagement {
    pub actor: FediverseActor,
    pub target: Option<String>,
}