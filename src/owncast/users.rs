use extism_pdk::Json;
use crate::errors::Forbidden;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;

pub fn list() -> Result<Vec<User>, Forbidden> {
    unsafe {
        crate::host::owncast_users_list().map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}

pub fn get(id: &str) -> Result<Option<User>, Forbidden> {
    unsafe {
        crate::host::owncast_user_get(id).map_err(|_| Forbidden)
    }
}

pub fn set_enabled(id: &str, enabled: bool, reason: Option<&str>) -> Result<(), Forbidden> {
    unsafe {
        crate::host::owncast_user_set_enabled(id, enabled, reason.unwrap_or("")).map_err(|_| Forbidden)
    }
}

pub fn ban_ip(ip: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::host::owncast_ban_ip(ip).map_err(|_| Forbidden)
    }
}

pub fn register(opts: UserRegisterRequest) -> Result<UserRegisterResult, Forbidden> {
    unsafe {
        crate::host::owncast_users_register(opts).map_err(|_| Forbidden)
    }
}