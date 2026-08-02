use extism_pdk::SharedFnResult;
use crate::host::{owncast_log_info, owncast_log_warning, owncast_log_error};

// Permissionless
pub fn info(message: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_log_info(message)
    }
}

// Permissionless
pub fn warning(message: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_log_warning(message)
    }
}

// Permissionless
pub fn error(message: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_log_error(message)
    }
}