use extism_pdk::FromBytesOwned;
use serde::Deserialize;

pub struct InputJson<T: for<'de> Deserialize<'de>>(pub T);

impl<T: for<'de> Deserialize<'de>> FromBytesOwned for InputJson<T> {
    fn from_bytes_owned(data: &[u8]) -> Result<Self, extism_pdk::Error> {
        let data = String::from_bytes_owned(data)?;
        let data: T = serde_json::from_str(data.as_str())?;
        Ok(InputJson(data))
    }
}