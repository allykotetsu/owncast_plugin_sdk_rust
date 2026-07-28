use std::error::Error;
use extism_pdk::Json;
use crate::errors::forbidden::Forbidden;
use crate::host::{owncast_fs_read, owncast_fs_write, owncast_fs_list, owncast_fs_delete, owncast_fs_exists};
use crate::json_objects::fs_result::FsResult;

pub fn read(path: &str) -> Result<Option<Vec<u8>>, Forbidden> {
    unsafe {
        owncast_fs_read(path).map_err(|_| Forbidden)
    }
}

pub fn read_text(path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let Some(vecu8) = read(path)? else {
        return Ok(None)
    };
    Ok(Some(String::from_utf8(vecu8)?))
}

pub fn write(path: &str, data: &[u8]) -> Result<FsResult, Forbidden> {
    unsafe {
        owncast_fs_write(path, data).map_err(|_| Forbidden)
    }
}

pub fn list(dir: &str) -> Result<Vec<String>, Forbidden> {
    unsafe {
        owncast_fs_list(dir).map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}

pub fn delete(path: &str) -> Result<FsResult, Forbidden> {
    unsafe {
        owncast_fs_delete(path).map_err(|_| Forbidden)
    }
}

pub fn exists(path: &str,) -> Result<bool, Forbidden> {
    unsafe {
        owncast_fs_exists(path).map(|i| i != 0).map_err(|_| Forbidden)
    }
}