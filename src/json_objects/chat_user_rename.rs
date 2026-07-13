use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatUserRename {
    pub(crate) user: User,
    pub(crate) previous_name: String
}