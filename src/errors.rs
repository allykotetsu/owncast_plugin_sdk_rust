use std::fmt::{Display, Formatter};
use serde::de::StdError;
use crate::json_objects::event_type::EventType;

pub struct Forbidden(pub(crate) String);
pub struct Duplicate(pub(crate) String);
pub struct OutOfBounds(pub(crate) String);

#[derive(Debug)]
pub struct MissingManifest;
impl Display for MissingManifest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Manifest could not be found.")
    }
}
impl StdError for MissingManifest {}

#[derive(Debug)]
pub struct BadEventType(pub EventType, pub EventType);
impl Display for BadEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BadEventType(expected, found) = self;
        write!(f, "Bad event type. Expected {expected}, found {found}.")
    }
}
impl StdError for BadEventType {}