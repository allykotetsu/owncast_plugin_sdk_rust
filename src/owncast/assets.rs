use extism_pdk::SharedFnResult;
use crate::host::owncast_asset_read;

/// Reads files the plugin shipped in its assets directory. Returns file contents as `Vec<u8>`.
///
/// # Errors
///
/// Errors if there is an issue setting memory in Extism.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # use owncast_plugin_sdk_rust::owncast;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let data = owncast::actions::read("data.json")?;
/// let data = String::from_utf8(data)?;
/// println!(&format!("{data}"));
/// #     Ok(())
/// # }
/// ```
pub fn read(path: &str) -> SharedFnResult<Option<Vec<u8>>> {
    unsafe {
        owncast_asset_read(path)
    }
}

/// Reads files the plugin shipped in its assets directory. Attempts to return file contents as `String`, if possible.
///
/// # Errors
///
/// Errors if there is an issue setting memory in Extism.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # use owncast_plugin_sdk_rust::owncast;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let data = owncast::actions::read_text("data.json")?;
/// println!(&format!("{data}"));
/// #     Ok(())
/// # }
/// ```
pub fn read_text(path: &str) -> SharedFnResult<Option<String>> {
    Ok(match read(path)? {
        Some(value) => Some(String::from_utf8(value)?),
        None => None
    })
}