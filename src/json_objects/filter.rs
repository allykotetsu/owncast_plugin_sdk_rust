use serde::Serialize;
use crate::json_objects::event::Event;

#[derive(Serialize)]
pub(crate) struct Filter {
    pub(crate) event: Event,
    pub(crate) priority: u8
}