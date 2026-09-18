use serde::Deserialize;
use crate::json_objects::user::User;

/// [`Envelope::ChatUserRenamed`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatUserRename {
    /// The [`User`] in question. Their new display name is [`User::display_name`].
    pub user: User,

    /// The previous display name that the [`User`] of this event had.
    pub previous_name: String
}