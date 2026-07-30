use extism_pdk::{Json, SharedFnResult};
use serde::Serialize;
use crate::host::{owncast_emit_event};

pub fn emit(event_type: &str, payload: &impl Serialize) -> SharedFnResult<()> {
    unsafe {
        owncast_emit_event(event_type, &Json(payload))
    }
}