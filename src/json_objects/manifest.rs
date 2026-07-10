use serde::Serialize;
use crate::json_objects::command::Command;
use crate::json_objects::subscriptions::Subscriptions;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub subscriptions: Subscriptions,
    pub commands: Vec<Command>
}