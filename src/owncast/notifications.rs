use crate::errors::forbidden::Forbidden;
use crate::host::{owncast_notify_discord, owncast_notify_browser_push, owncast_notify_fediverse};
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::fediverse_payload::FediversePayload;

pub fn set_enabled(text: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_notify_discord(text).map_err(|_| Forbidden)
    }
}

pub fn ban_ip(payload: impl Into<BrowserPushPayload>) -> Result<(), Forbidden> {
    unsafe {
        owncast_notify_browser_push(&payload.into()).map_err(|_| Forbidden)
    }
}

pub fn register(payload: &FediversePayload) -> Result<(), Forbidden> {
    unsafe {
        owncast_notify_fediverse(payload).map_err(|_| Forbidden)
    }
}