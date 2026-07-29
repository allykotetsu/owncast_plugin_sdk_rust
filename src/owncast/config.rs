use extism_pdk::{Json, SharedFnResult};
use serde::de::DeserializeOwned;
use crate::host::owncast_config_get;

pub fn get<T: DeserializeOwned>(key: &str) -> SharedFnResult<Option<T>> {
    unsafe {
        let Some(Json(value)) = owncast_config_get(key)? else {
            return Ok(None);
        };
        Ok(value)
    }
}