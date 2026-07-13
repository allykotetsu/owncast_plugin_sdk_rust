use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatMessageModeration {
    message_id: String,
    visible: bool,
    moderator: Option<User>
}