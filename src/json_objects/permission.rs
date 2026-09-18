use serde::{Deserialize, Serialize};

/// Permissions as defined in the `plugin.manifest.json`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Permission {
    /// The [`Permission`] required to send [`ChatMessage`]s.
    #[serde(rename = "chat.send")]
    ChatSend,

    /// The [`Permission`] required to read the [`ChatMessage`] history.
    #[serde(rename = "chat.history")]
    ChatHistory,

    /// The [`Permission`] required to moderate [`ChatMessage`]s.
    #[serde(rename = "chat.moderate")]
    ChatModerate,

    /// The [`Permission`] required to filter [`ChatMessage`]s.
    #[serde(rename = "chat.filter")]
    ChatFilter,

    /// The [`Permission`] required to read [`User`] info.
    #[serde(rename = "users.read")]
    UsersRead,

    /// The [`Permission`] required to moderate [`User`]s.
    #[serde(rename = "users.moderate")]
    UsersModerate,

    /// The [`Permission`] required to register new [`User`]s.
    #[serde(rename = "users.register")]
    UsersRegister,

    /// The [`Permission`] required to lock Owncast behind an authorization gate.
    #[serde(rename = "auth.gate")]
    AuthGate,

    /// The [`Permission`] required to store data in key-value memory.
    #[serde(rename = "storage.kv")]
    StorageKV,

    /// The [`Permission`] required to upload public files to the plugin's data folders.
    #[serde(rename = "storage.upload")]
    StorageUpload,

    /// The [`Permission`] required to manage the filesystem the plugin owns.
    #[serde(rename = "storage.fs")]
    StorageFs,

    /// The [`Permission`] required to manipulate the SQLite database the plugin runs.
    #[serde(rename = "storage.sql")]
    StorageSql,

    /// The [`Permission`] required to make HTTP requests.
    #[serde(rename = "network.fetch")]
    NetworkFetch,

    /// The [`Permission`] required to emit custom event data.
    #[serde(rename = "events.emit")]
    EventsEmit,

    /// The [`Permission`] required to serve HTTP endpoints.
    #[serde(rename = "http.serve")]
    HttpServe,

    /// The [`Permission`] required to communicate via Server-Sent Events.
    #[serde(rename = "http.sse")]
    HttpSse,

    /// The [`Permission`] required to read stream state and server config.
    #[serde(rename = "server.read")]
    ServerRead,

    /// The [`Permission`] required to read video config data.
    #[serde(rename = "videoconfig.read")]
    VideoConfigRead,

    /// The [`Permission`] required to alter video config data.
    #[serde(rename = "videoconfig.write")]
    VideoConfigWrite,

    /// The [`Permission`] required to send notifications to Users.
    #[serde(rename = "notifications.send")]
    NotificationsSend,

    /// The [`Permission`] required to receive information on inbound fediverse activities.
    #[serde(rename = "fediverse.inbound")]
    FediverseInbound,

    /// The [`Permission`] required to post outbound fediverse activities.
    #[serde(rename = "fediverse.post")]
    FediversePost,

    /// The [`Permission`] required to modify Owncast's UI, CSS, and JS.
    #[serde(rename = "ui.modify")]
    UiModify
}