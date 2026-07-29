use extism_pdk::SharedFnResult;
use crate::host::{owncast_auth_grant_session, owncast_auth_end_session};
use crate::json_objects::error::Error;
use crate::json_objects::grant_session_request::GrantSessionRequest;

pub fn grant_session(opts: impl Into<GrantSessionRequest>) -> SharedFnResult<Error> {
    unsafe {
        owncast_auth_grant_session(&opts.into())
    }
}

pub fn end_session() -> SharedFnResult<()> {
    unsafe {
        owncast_auth_end_session()
    }
}