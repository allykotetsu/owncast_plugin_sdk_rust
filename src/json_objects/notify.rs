use serde::Serialize;
use crate::json_objects::event::Event;

#[derive(Serialize)]
pub(crate) struct Notify {
    pub(crate) event: Event
}