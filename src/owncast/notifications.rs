use extism_pdk::SharedFnResult;
use crate::host::{owncast_notify_discord, owncast_notify_browser_push, owncast_notify_fediverse};
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::fediverse_payload::FediversePayload;

pub fn discord(text: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_notify_discord(text)
    }
}

pub fn browser_push(payload: impl Into<BrowserPushPayload>) -> SharedFnResult<()> {
    let payload = &payload.into();
    unsafe {
        owncast_notify_browser_push(payload)
    }
}

pub fn fediverse(payload: &FediversePayload) -> SharedFnResult<()> {
    unsafe {
        owncast_notify_fediverse(payload)
    }
}