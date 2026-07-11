use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::envelope::Envelope;

pub enum FilterResult {
    Pass,
    Modify(ChatMessage),
    Drop(String)
}

impl Serialize for FilterResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}