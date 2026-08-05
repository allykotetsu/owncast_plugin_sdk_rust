use anyhow::{anyhow, Error};
use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ActionResult {
    pub error: Option<String>
}

impl TryInto<()> for ActionResult {
    type Error = Error;

    fn try_into(self) -> Result<(), Self::Error> {
        match self.error {
            None => Ok(()),
            Some(err) => Err(anyhow!(err))
        }
    }
}