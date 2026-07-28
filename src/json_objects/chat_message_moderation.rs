use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageModeration {
    pub message_id: String,
    pub visible: bool,
    pub moderator: Option<User>
}