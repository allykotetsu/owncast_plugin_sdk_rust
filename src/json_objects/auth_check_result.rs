use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(ToBytes, Serialize, Clone, Debug)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum AuthCheckResult {
    Ok,
    Refresh { ttl: Option<u64> },
    Deny { reason: String }
}

impl AuthCheckResult {
    pub fn ok() -> AuthCheckResult {
        AuthCheckResult::Ok
    }

    pub fn refresh() -> AuthCheckResult {
        AuthCheckResult::Refresh {
            ttl: None
        }
    }

    pub fn refresh_ttl(ttl: u64) -> AuthCheckResult {
        AuthCheckResult::Refresh {
            ttl: Some(ttl)
        }
    }

    pub fn deny(reason: &str) -> AuthCheckResult {
        AuthCheckResult::Deny {
            reason: reason.to_string()
        }
    }
}