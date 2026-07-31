use serde::Serialize;
use crate::json_objects::event_type::EventType;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Notify {
    pub(crate) event: EventType
}