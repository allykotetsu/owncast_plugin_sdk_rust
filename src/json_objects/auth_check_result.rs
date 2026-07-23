use extism_pdk::{ToBytes, Json};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(ToBytes)]
#[encoding(Json)]
pub enum AuthCheckResult {
    Ok,
    Refresh(Option<u64>),
    Deny(Option<String>)
}

impl Serialize for AuthCheckResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state;
        match self {
            AuthCheckResult::Ok => {
                state = serializer.serialize_struct("", 1)?;
                state.serialize_field("action", "ok")?;
            }
            AuthCheckResult::Refresh(opts) => {
                state = serializer.serialize_struct("", if opts.is_some() { 2 } else { 1 })?;
                state.serialize_field("action", "refresh")?;
                if let Some(opts) = opts {
                    state.serialize_field("ttl", opts)?;
                }
            }
            AuthCheckResult::Deny(reason) => {
                state = serializer.serialize_struct("", 2)?;
                state.serialize_field("action", "deny")?;
                state.serialize_field("reason", &reason.clone().unwrap_or(String::new()))?;
            }
        }
        state.end()
    }
}