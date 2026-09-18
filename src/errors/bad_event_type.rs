use std::error::Error;
use crate::json_objects::event_type::EventType;
use std::fmt::{Display, Formatter};

/// An error for when a filter call uses an event type that is not [`EventType::ChatMessageReceived`]
#[derive(Debug, Clone)]
pub struct BadEventType(pub EventType, pub EventType);

impl Display for BadEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BadEventType(expected, found) = self;
        write!(f, "Bad event type. Expected {expected}, found {found}.")
    }
}

impl Error for BadEventType {}