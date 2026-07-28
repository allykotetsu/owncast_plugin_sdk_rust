use crate::errors::forbidden::Forbidden;
use crate::host::{owncast_auth_grant_session, owncast_auth_end_session};
use crate::json_objects::error::Error;
use crate::json_objects::grant_session_request::GrantSessionRequest;

pub fn grant_session(opts: impl Into<GrantSessionRequest>) -> Result<Error, Forbidden> {
    unsafe {
        owncast_auth_grant_session(&opts.into()).map_err(|_| Forbidden)
    }
}

pub fn end_session() -> Result<(), Forbidden> {
    unsafe {
        owncast_auth_end_session().map_err(|_| Forbidden)
    }
}