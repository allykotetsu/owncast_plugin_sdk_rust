use serde::{Deserialize, Deserializer};
use crate::json_objects::event::Event;

pub struct Envelope {
    pub(crate) event_type: Event
}

impl<'de> Deserialize<'de> for Envelope {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

// TODO custom deserialize logic. Take JSON field "payload" and move data to Event parameter
// Refer to https://serde.rs/impl-deserialize.html for documentation.