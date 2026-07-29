use extism_pdk::SharedFnResult;
use crate::host::{owncast_notify_discord, owncast_notify_browser_push, owncast_notify_fediverse};
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::fediverse_payload::FediversePayload;

pub fn set_enabled(text: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_notify_discord(text)
    }
}

pub fn ban_ip(payload: impl Into<BrowserPushPayload>) -> SharedFnResult<()> {
    unsafe {
        owncast_notify_browser_push(&payload.into())
    }
}

pub fn register(payload: &FediversePayload) -> SharedFnResult<()> {
    unsafe {
        owncast_notify_fediverse(payload)
    }
}