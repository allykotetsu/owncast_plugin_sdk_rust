use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct UserRegisterRequest {
    pub auth_id: String,
    pub display_name: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub profile_url: Option<String>,
    pub handle: Option<String>,
    pub public: Option<bool>,
}

impl From<&str> for UserRegisterRequest {
    fn from(s: &str) -> UserRegisterRequest {
        UserRegisterRequest {
            auth_id: s.to_string(),
            display_name: None,
            scopes: None,
            profile_url: None,
            handle: None,
            public: None,
        }
    }
}