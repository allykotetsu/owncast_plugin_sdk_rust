use extism_pdk::SharedFnResult;
use crate::host::{owncast_auth_grant_session, owncast_auth_end_session};
use crate::json_objects::grant_session_request::GrantSessionRequest;

pub fn grant_session(opts: impl Into<GrantSessionRequest>) -> SharedFnResult<()> {
    let opts = &opts.into();
    let action_result = unsafe {
        owncast_auth_grant_session(opts)
    };
    action_result?.try_into()
}

pub fn end_session() -> SharedFnResult<()> {
    unsafe {
        owncast_auth_end_session()
    }
}