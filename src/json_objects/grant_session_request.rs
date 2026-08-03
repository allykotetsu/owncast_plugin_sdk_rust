use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct GrantSessionRequest {
    pub user_id: String,
    pub ttl: Option<i64>
}

impl GrantSessionRequest {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            ttl: None,
        }
    }

    pub fn with_ttl(mut self, ttl: i64) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

impl From<&str> for GrantSessionRequest {
    fn from(s: &str) -> Self {
        GrantSessionRequest {
            user_id: s.to_string(),
            ttl: None
        }
    }
}