use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub(crate) enum Permission {
    #[serde(rename = "chat.send")]
    ChatSend,
    #[serde(rename = "chat.history")]
    ChatHistory,
    #[serde(rename = "chat.moderate")]
    ChatModerate,
    #[serde(rename = "chat.filter")]
    ChatFilter,

    #[serde(rename = "users.read")]
    UsersRead,
    #[serde(rename = "users.moderate")]
    UsersModerate,
    #[serde(rename = "users.register")]
    UsersRegister,

    #[serde(rename = "auth.gate")]
    AuthGate,

    #[serde(rename = "storage.kv")]
    StorageKV,
    #[serde(rename = "storage.upload")]
    StorageUpload,
    #[serde(rename = "storage.fs")]
    StorageFS,

    #[serde(rename = "network.fetch")]
    NetworkFetch,

    #[serde(rename = "events.emit")]
    EventsEmit,

    #[serde(rename = "http.serve")]
    HttpServe,
    #[serde(rename = "http.sse")]
    HttpSse,

    #[serde(rename = "server.read")]
    ServerRead,

    #[serde(rename = "videoconfig.read")]
    VideoConfigRead,
    #[serde(rename = "videoconfig.write")]
    VideoConfigWrite,

    #[serde(rename = "notifications.send")]
    NotificationsSend,

    #[serde(rename = "fediverse.post")]
    FediversePost,

    #[serde(rename = "ui.modify")]
    UiModify
}

/*/// # Examples
/// ```
/// #[permitted(Permission::ChatSend)]
/// fn do_thing(_: &str) -> bool;
/// ```
///
/// expands to
/// ```
/// fn do_thing_(_: &str) -> bool;
///
/// pub fn do_thing(arg_0: &str) -> Result<bool, PermissionsError> {
///     if PLUGIN.permitted(Permission::ChatSend) {
///         Ok(do_thing_(arg_0))
///     } else {
///         Err(PermissionsError)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn permitted(attr: TokenStream, item: TokenStream) -> TokenStream {
    todo!()
}*/