use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatUserRename {
    pub user: User,
    pub previous_name: String
}