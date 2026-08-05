use extism_pdk::SharedFnResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::errors::key_not_found::KeyNotFound;
use crate::host::{owncast_kv_get, owncast_kv_set};

pub fn get(key: &str) -> SharedFnResult<Option<String>> {
    unsafe {
        owncast_kv_get(key)
    }
}

pub fn set<'a>(key: &str, value: impl Into<&'a str>) -> SharedFnResult<()> {
    let value = &value.into();
    let action_result = unsafe {
        owncast_kv_set(key, value)
    };
    action_result?.try_into()
}

pub fn get_json<T: DeserializeOwned>(key: &str) -> SharedFnResult<Option<T>> {
    let Some(value) = get(key)? else {
        return Err(KeyNotFound(key.to_string()).into());
    };
    Ok(Some(serde_json::from_str(&value)?))
}

pub fn set_json(key: &str, value: impl Serialize) -> SharedFnResult<()> {
    set(key, serde_json::to_string(&value)?.as_str())
}