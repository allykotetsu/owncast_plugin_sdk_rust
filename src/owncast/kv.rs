use std::error::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::errors::forbidden::Forbidden;
use crate::errors::key_not_found::KeyNotFound;
use crate::host::{owncast_kv_get, owncast_kv_set};

pub fn get(key: &str) -> Result<Option<String>, Forbidden> {
    unsafe {
        owncast_kv_get(key).map_err(|_| Forbidden)
    }
}

pub fn set<'a>(key: &str, value: impl Into<&'a str>) -> Result<(), Forbidden> {
    unsafe {
        owncast_kv_set(key, value.into()).map_err(|_| Forbidden)
    }
}

pub fn get_json<T: DeserializeOwned>(key: &str) -> Result<Option<T>, Box<dyn Error>> {
    let Some(value) = get(key)? else {
        return Err(Box::new(KeyNotFound(key.to_string())));
    };
    Ok(Some(serde_json::from_str(&value)?))
}

pub fn set_json(key: &str, value: impl Serialize) -> Result<(), Box<dyn Error>> {
    Ok(set(key, serde_json::to_string(&value)?.as_str())?)
}