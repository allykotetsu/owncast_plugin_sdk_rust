use extism_pdk::{Json, SharedFnResult};
use serde::Serialize;
use crate::host::{owncast_emit_event};

pub fn emit(ns: &str, event_type: &str, payload: &impl Serialize) -> SharedFnResult<()> {
    let event = format!("{ns}.{event_type}");
    let event = event.as_str();
    let payload = &Json(serde_json::to_string(payload)?);
    unsafe {
        owncast_emit_event(event, payload)
    }
}