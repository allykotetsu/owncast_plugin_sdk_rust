use anyhow::anyhow;
use extism_pdk::SharedFnResult;
use crate::host::{owncast_auth_grant_session, owncast_auth_end_session};
use crate::json_objects::grant_session_request::GrantSessionRequest;

pub fn grant_session(opts: impl Into<GrantSessionRequest>) -> SharedFnResult<()> {
    let res = unsafe {
        owncast_auth_grant_session(&opts.into())?
    };
    match res.error {
        Some(error) => Err(anyhow!(error)),
        None => Ok(())
    }
}

pub fn end_session() -> SharedFnResult<()> {
    unsafe {
        owncast_auth_end_session()
    }
}