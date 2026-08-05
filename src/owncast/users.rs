use std::net::Ipv4Addr;
use extism_pdk::{Json, Memory, SharedFnResult};
use crate::errors::mem_not_found::MemNotFound;
use crate::host::{owncast_users_list, owncast_user_get, owncast_ban_ip, owncast_user_set_enabled, owncast_users_register};
use crate::json_objects::action_result::ActionResult;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;

pub fn list() -> SharedFnResult<Vec<User>> {
    let res = unsafe {
        owncast_users_list()
    };
    res.map(|Json(inner)| inner)
}

pub fn get(id: &str) -> SharedFnResult<Option<User>> {
    unsafe {
        owncast_user_get(id)
    }
}

pub fn set_enabled(id: &str, enabled: bool, reason: Option<&str>) -> SharedFnResult<()> {
    let id = Memory::new(&id)?.offset();
    let enabled = enabled as i64;
    let reason = Memory::new(&reason.unwrap_or(""))?.offset();

    let offset = unsafe {
        owncast_user_set_enabled(id, enabled, reason)
    };
    let memory = Memory::find(offset).ok_or(MemNotFound)?;
    let action_result: ActionResult = memory.to()?;
    action_result.try_into()
}

pub fn ban_ip(ip: &Ipv4Addr) -> SharedFnResult<()> {
    let ip = &ip.to_string();
    let action_result = unsafe {
        owncast_ban_ip(ip)
    };
    action_result?.try_into()
}

pub fn register(opts: impl Into<UserRegisterRequest>) -> SharedFnResult<String> {
    let opts = &opts.into();
    let res = unsafe {
        owncast_users_register(opts)
    };
    res?.try_into()
}