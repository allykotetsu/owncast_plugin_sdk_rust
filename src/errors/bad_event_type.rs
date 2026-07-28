use std::error::Error;
use crate::json_objects::event_type::EventType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct BadEventType(pub EventType, pub EventType);

impl Display for BadEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BadEventType(expected, found) = self;
        write!(f, "Bad event type. Expected {expected}, found {found}.")
    }
}

impl Error for BadEventType {}