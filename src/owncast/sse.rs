use extism_pdk::{Json, SharedFnResult};
use serde::Serialize;
use crate::host::owncast_sse_send;

pub fn send(channel: &str, event: &str, data: &impl Serialize) -> SharedFnResult<()> {
    unsafe {
        owncast_sse_send(channel, event, &Json(data))
    }
}