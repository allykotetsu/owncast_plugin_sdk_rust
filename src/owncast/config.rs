use std::error::Error;
use extism_pdk::Json;
use serde::de::DeserializeOwned;
use crate::host::owncast_config_get;

pub fn get<T: DeserializeOwned>(key: &str) -> Result<T, Box<dyn Error>> {
    unsafe {
        let Json(value) = owncast_config_get(key)?;
        Ok(value)
    }
}