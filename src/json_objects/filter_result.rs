use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub enum FilterResult {
    Pass,
    Modify(String),
    Drop(Option<String>)
}

impl Serialize for FilterResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state;
        match self {
            FilterResult::Pass => {
                state = serializer.serialize_struct("", 1)?;
                state.serialize_field("action", "pass")?;
            }
            FilterResult::Modify(body) => {
                // TODO build entire chat message object from body.
                // So that the plugin author is encouraged to only edit the message body but they can find a way around that if they really need to edit more.
                state = serializer.serialize_struct("", 2)?;
                state.serialize_field("action", "modify")?;
                state.serialize_field("payload", body)?;
            }
            FilterResult::Drop(reason) => {
                state = serializer.serialize_struct("", 2)?;
                state.serialize_field("action", "drop")?;
                state.serialize_field("reason", &reason.clone().unwrap_or(String::new()))?;
            }
        }
        state.end()
    }
}