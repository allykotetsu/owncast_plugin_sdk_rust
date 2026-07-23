use std::fmt::{Display, Formatter};
use serde::de::StdError;

pub struct Forbidden(pub(crate) String);
pub struct Duplicate(pub(crate) String);
pub struct OutOfBounds(pub(crate) String);
#[derive(Debug)]
pub struct BadEventType(pub String);

impl Display for BadEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let BadEventType(reason) = self;
        write!(f, "{reason}")
    }
}
impl StdError for BadEventType {}