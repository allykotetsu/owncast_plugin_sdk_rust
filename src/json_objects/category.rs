use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Category {
    ChatBots,
    ChatFilters,
    Moderation,
    Authentication,
    Themes,
    Overlaps,
    Notifications,
    Integrations,
    Video,
    Analytics,
    Games,
    AdminUtilities,
    Examples,
    Other
}