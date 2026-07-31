use anyhow::anyhow;
use extism_pdk::{Json, SharedFnResult};
use crate::host::{owncast_fs_read, owncast_fs_write, owncast_fs_list, owncast_fs_delete, owncast_fs_exists};

pub fn read(path: &str) -> SharedFnResult<Option<Vec<u8>>> {
    unsafe {
        owncast_fs_read(path)
    }
}

pub fn read_text(path: &str) -> SharedFnResult<Option<String>> {
    let Some(vecu8) = read(path)? else {
        return Ok(None)
    };
    Ok(Some(String::from_utf8(vecu8)?))
}

pub fn write(path: &str, data: impl Into<Vec<u8>>) -> SharedFnResult<()> {
    let res = unsafe {
        owncast_fs_write(path, &data.into())?
    };
    match res.error {
        Some(error) => Err(anyhow!(error)),
        None => Ok(())
    }
}

pub fn list(dir: &str) -> SharedFnResult<Vec<String>> {
    unsafe {
        owncast_fs_list(dir).map(|Json(inner)| inner)
    }
}

pub fn delete(path: &str) -> SharedFnResult<()> {
    let res = unsafe {
        owncast_fs_delete(path)?
    };
    match res.error {
        Some(error) => Err(anyhow!(error)),
        None => Ok(())
    }
}

pub fn exists(path: &str) -> SharedFnResult<bool> {
    unsafe {
        owncast_fs_exists(path).map(|i| i != 0)
    }
}