use std::fmt::Debug;
use crate::owncast;

/// A convenience macro for logging an error through Extism when one arises.
#[macro_export]
macro_rules! run {
    ($func:expr) => {
        match $func {
            Err(err) => extism_pdk::error!("{err}"),
            _ => {}
        }
    };
}

pub fn debug(val: impl Debug) {
    run!(owncast::chat::send(&format!("{val:#?}")));
}