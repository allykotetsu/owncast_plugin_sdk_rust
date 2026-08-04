use std::net::Ipv4Addr;
use anyhow::anyhow;
use extism_pdk::{Json, Memory, SharedFnResult};
use crate::errors::mem_not_found::MemNotFound;
use crate::host::{owncast_users_list, owncast_user_get, owncast_ban_ip, owncast_user_set_enabled, owncast_users_register};
use crate::json_objects::action_result::ActionResult;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;

pub fn list() -> SharedFnResult<Vec<User>> {
    unsafe {
        owncast_users_list().map(|Json(inner)| inner)
    }
}

pub fn get(id: &str) -> SharedFnResult<Option<User>> {
    unsafe {
        owncast_user_get(id)
    }
}

pub fn set_enabled(id: &str, enabled: bool, reason: Option<&str>) -> SharedFnResult<()> {
    let id = Memory::new(&id)?.offset();
    let reason = Memory::new(&reason.unwrap_or(""))?.offset();

    let offset = unsafe {
        owncast_user_set_enabled(id, enabled as i64, reason)
    };
    let memory = Memory::find(offset).ok_or(MemNotFound)?;
    let action_result: ActionResult = memory.to()?;

    if let Some(err) = action_result.error {
        Err(anyhow!(err))
    } else {
        Ok(())
    }
}

pub fn ban_ip(ip: &Ipv4Addr) -> SharedFnResult<()> {
    unsafe {
        if let Some(err) = owncast_ban_ip(&ip.to_string())?.error {
            Err(anyhow!(err))
        } else {
            Ok(())
        }
    }
}

pub fn register(opts: impl Into<UserRegisterRequest>) -> SharedFnResult<String> {
    let res = unsafe {
        owncast_users_register(&opts.into())?
    };
    res.user_id.ok_or(anyhow!(res.error.unwrap_or("There was an error registering the user.".to_string())))
}