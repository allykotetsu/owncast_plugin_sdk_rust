use extism_pdk::{Error, Memory, ToMemory};
use serde::Serialize;

pub struct OutputJson<T: Serialize>(pub T);

impl<T: Serialize> ToMemory for &OutputJson<T> {
    fn to_memory(&self) -> Result<Memory, Error> {
        let OutputJson(value) = self;
        let value = serde_json::to_string(&value)?;
        value.to_memory()
    }
}