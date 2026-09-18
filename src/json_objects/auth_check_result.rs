use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// The result of an authorization check.
#[derive(ToBytes, Serialize, Clone, Debug)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum AuthCheckResult {
    /// The user of the [`AuthCheckResult`] is authorized.
    Ok,

    /// The user of the [`AuthCheckResult`] is authorized and their cookies are extended.
    Refresh {
        /// Time-to-live in seconds that the new session is valid for.
        ttl: Option<u64>
    },

    /// End the session of the user of the [`AuthCheckResult`].
    Deny {
        /// Why the session was denied.
        reason: String
    }
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