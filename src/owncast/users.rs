use std::net::Ipv4Addr;
use extism_pdk::Json;
use crate::errors::forbidden::Forbidden;
use crate::host::{owncast_users_list, owncast_user_get, owncast_ban_ip, owncast_user_set_enabled, owncast_users_register};
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;

pub fn list() -> Result<Vec<User>, Forbidden> {
    unsafe {
        owncast_users_list().map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}

pub fn get(id: &str) -> Result<Option<User>, Forbidden> {
    unsafe {
        owncast_user_get(id).map_err(|_| Forbidden)
    }
}

pub fn set_enabled(id: &str, enabled: bool, reason: Option<&str>) -> Result<(), Forbidden> {
    unsafe {
        owncast_user_set_enabled(id, enabled, reason.unwrap_or("")).map_err(|_| Forbidden)
    }
}

pub fn ban_ip(ip: &Ipv4Addr) -> Result<(), Forbidden> { // TODO make IP struct
    unsafe {
        owncast_ban_ip(&ip.to_string()).map_err(|_| Forbidden)
    }
}

pub fn register(opts: impl Into<UserRegisterRequest>) -> Result<UserRegisterResult, Forbidden> {
    unsafe {
        owncast_users_register(&opts.into()).map_err(|_| Forbidden)
    }
}