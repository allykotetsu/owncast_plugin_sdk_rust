use serde::Serialize;
use crate::json_objects::event_type::EventType;

#[derive(Serialize, Clone, Debug)]
pub(crate) struct Filter {
    pub(crate) event: EventType,
    pub(crate) priority: u8
}