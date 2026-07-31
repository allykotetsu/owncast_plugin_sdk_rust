use extism_pdk::SharedFnResult;
use crate::host::owncast_asset_read;

// Does not require permissions.
pub fn read(path: &str) -> SharedFnResult<Option<Vec<u8>>> {
    unsafe {
        owncast_asset_read(path)
    }
}

// Does not require permissions.
pub fn read_text(path: &str) -> SharedFnResult<Option<String>> {
    Ok(match read(path)? {
        Some(value) => Some(String::from_utf8(value)?),
        None => None
    })
}