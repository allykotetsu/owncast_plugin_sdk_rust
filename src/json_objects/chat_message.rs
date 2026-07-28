use extism_pdk::{FromBytes, Json};
use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

#[derive(Serialize, Deserialize, Clone, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatMessage {
    pub id: String,
    pub user: Option<User>,
    pub client_id: Option<i64>,
    pub body: String,
    pub timestamp: String
}

impl TryInto<i64> for &ChatMessage {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        match self.client_id {
            None => Err(()),
            Some(client_id) => Ok(client_id)
        }
    }
}