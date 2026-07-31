use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct OutOfBounds<T: Debug + Display>(pub T, pub T, pub T);

impl<T: Debug + Display> Display for OutOfBounds<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let OutOfBounds(min, max, num) = self;
        write!(f, "Number must be between {min} (inclusive) and {max} (exclusive), found {num}.")
    }
}

impl<T: Debug + Display> Error for OutOfBounds<T> {}