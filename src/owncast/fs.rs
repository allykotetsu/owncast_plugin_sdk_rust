use extism_pdk::{Json, Memory, SharedFnResult};
use crate::host::{owncast_fs_read, owncast_fs_write, owncast_fs_list, owncast_fs_delete, owncast_fs_exists};

pub fn read(path: &str) -> SharedFnResult<Option<Vec<u8>>> {
    unsafe {
        owncast_fs_read(path)
    }
}

pub fn read_text(path: &str) -> SharedFnResult<Option<String>> {
    Ok(match read(path)? {
        Some(value) => Some(String::from_utf8(value)?),
        None => None
    })
}

pub fn write(path: &str, data: impl Into<Vec<u8>>) -> SharedFnResult<()> {
    let data = &data.into();
    let action_result = unsafe {
        owncast_fs_write(path, data)
    };
    action_result?.try_into()
}

pub fn list(dir: &str) -> SharedFnResult<Vec<String>> {
    let res = unsafe {
        owncast_fs_list(dir)
    };
    res.map(|Json(inner)| inner)
}

pub fn delete(path: &str) -> SharedFnResult<()> {
    let action_result = unsafe {
        owncast_fs_delete(path)
    };
    action_result?.try_into()
}

pub fn exists(path: &str) -> SharedFnResult<bool> {
    let path = Memory::from_bytes(path.as_bytes())?.offset();
    let res = unsafe {
        owncast_fs_exists(path)
    };
    Ok(res != 0)
}