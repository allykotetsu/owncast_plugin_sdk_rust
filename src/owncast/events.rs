use std::error::Error;
use extism_pdk::Json;
use serde::Serialize;
use crate::host::{owncast_emit_event};

pub fn emit(event_type: &str, payload: impl Serialize) -> Result<(), Box<dyn Error>> {
    unsafe {
        Ok(owncast_emit_event(event_type, Json(payload))?)
    }
}