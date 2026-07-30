use extism_pdk::{Json, SharedFnResult};
use crate::host::{owncast_server_info, owncast_server_socials, owncast_server_emotes, owncast_server_federation, owncast_server_tags};
use crate::json_objects::emote::Emote;
use crate::json_objects::federation_info::FederationInfo;
use crate::json_objects::server_info::ServerInfo;
use crate::json_objects::social_handle::SocialHandle;

pub fn info() -> SharedFnResult<ServerInfo> {
    unsafe {
        owncast_server_info()
    }
}

pub fn socials() -> SharedFnResult<Vec<SocialHandle>> {
    unsafe {
        owncast_server_socials().map(|Json(inner)| inner)
    }
}

pub fn emotes() -> SharedFnResult<Vec<Emote>> {
    unsafe {
        owncast_server_emotes().map(|Json(inner)| inner)
    }
}

pub fn federation() -> SharedFnResult<FederationInfo> {
    unsafe {
        owncast_server_federation()
    }
}

pub fn tags() -> SharedFnResult<Vec<String>> {
    unsafe {
        owncast_server_tags().map(|Json(inner)| inner)
    }
}