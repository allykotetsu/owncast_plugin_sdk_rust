use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// An object representing a request to Owncast to grant all sessions pertaining to a specific [`User`].
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct GrantSessionRequest {
    /// The ID of the [`User`] who is being granted a session.
    pub user_id: String,

    /// The time-to-live (in seconds) that session is valid for.
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

impl From<String> for GrantSessionRequest {
    fn from(s: String) -> Self {
        GrantSessionRequest {
            user_id: s,
            ttl: None
        }
    }
}