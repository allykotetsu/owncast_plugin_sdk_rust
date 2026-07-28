use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct UserRegisterRequest {
    pub auth_id: String,
    pub display_name: Option<String>,
    pub scopes: Option<Vec<String>>
}

impl Into<UserRegisterRequest> for &str {
    fn into(self) -> UserRegisterRequest {
        UserRegisterRequest {
            auth_id: self.to_string(),
            display_name: None,
            scopes: None
        }
    }
}