use serde::Deserialize;
use crate::json_objects::event::Event;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelope<T> {
    pub(crate) event_type: Event,
    pub(crate) payload: T
}