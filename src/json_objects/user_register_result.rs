use anyhow::{anyhow, Error};
use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct UserRegisterResult {
    pub user_id: Option<String>,
    pub error: Option<String>
}

impl TryInto<String> for UserRegisterResult {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        self.user_id.ok_or(anyhow!(self.error.unwrap_or("There was an error registering the user.".to_string())))
    }
}