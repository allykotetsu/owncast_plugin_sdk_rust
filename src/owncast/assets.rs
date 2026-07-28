use std::error::Error;
use crate::host::owncast_asset_read;

pub fn read(path: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    unsafe {
        Ok(owncast_asset_read(path)?)
    }
}

pub fn read_text(path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let Some(value) = read(path)? else {
        return Ok(None)
    };
    Ok(Some(String::from_utf8(value)?))
}