use std::fmt::{Debug, Display, Formatter};
use serde::de::StdError;
use crate::json_objects::event_type::EventType;

pub struct Forbidden;

#[derive(Debug)]
pub struct Duplicate(pub(crate) String);
impl Display for Duplicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Duplicate(key) = self;
        write!(f, "Duplicate entry for {key}.")
    }
}
impl StdError for Duplicate {}

#[derive(Debug)]
pub struct OutOfBounds<T: Debug + Display>(pub T, pub T, pub T);
impl<T: Debug + Display> Display for OutOfBounds<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let OutOfBounds(min, max, num) = self;
        write!(f, "Number must be between {min} (inclusive) and {max} (exclusive), found {num}.")
    }
}
impl<T: Debug + Display> StdError for OutOfBounds<T> {}

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

#[derive(Debug)]
pub struct Dbg(pub String);
impl Display for Dbg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Dbg(msg) = self;
        write!(f, "{msg}")
    }
}
impl StdError for Dbg {}