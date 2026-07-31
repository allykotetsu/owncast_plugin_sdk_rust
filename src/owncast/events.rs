use extism_pdk::{Json, SharedFnResult};
use serde::Serialize;
use crate::host::{owncast_emit_event};

pub fn emit(ns: &str, event_type: &str, payload: &impl Serialize) -> SharedFnResult<()> {
    unsafe {
        owncast_emit_event(format!("{ns}:{event_type}").as_str(), &Json(serde_json::to_string(payload)?))
    }
}