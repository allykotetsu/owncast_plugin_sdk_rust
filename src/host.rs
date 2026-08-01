use extism_pdk::{host_fn, Json};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::json_objects::action_button::ActionButton;
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::emote::Emote;
use crate::json_objects::auth_result::AuthResult;
use crate::json_objects::federation_info::FederationInfo;
use crate::json_objects::fediverse_payload::FediversePayload;
use crate::json_objects::fs_result::FsResult;
use crate::json_objects::grant_session_request::GrantSessionRequest;
use crate::json_objects::server_info::ServerInfo;
use crate::json_objects::social_handle::SocialHandle;
use crate::json_objects::sql_exec_result::SqlExecResult;
use crate::json_objects::sql_query_result::SqlQueryResult;
use crate::json_objects::sql_request::SqlRequest;
use crate::json_objects::stream_broadcaster::StreamBroadcaster;
use crate::json_objects::stream_info::StreamInfo;
use crate::json_objects::upload_result::UploadResult;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;
use crate::json_objects::video_config::VideoConfig;
use crate::json_objects::video_config_update::VideoConfigUpdate;

#[host_fn]
extern "ExtismHost" {
    // Chat
    pub(crate) fn owncast_send_chat(textPtr: &str);
    pub(crate) fn owncast_send_chat_action(textPtr: &str);
    pub(crate) fn owncast_send_chat_system(bodyPtr: &str);
    pub(crate) fn owncast_send_chat_to(clientId: i64, textPtr: &str); // TODO
    pub(crate) fn owncast_chat_history(limit: i64) -> Json<Vec<ChatMessage>>; // TODO
    pub(crate) fn owncast_delete_message(idPtr: &str);
    pub(crate) fn owncast_kick_client(clientId: i64); // TODO
    pub(crate) fn owncast_chat_clients() -> Json<Vec<ChatClient>>;

    // Users
    pub(crate) fn owncast_users_list() -> Json<Vec<User>>;
    pub(crate) fn owncast_user_get(idPtr: &str) -> Option<User>;
    pub(crate) fn owncast_user_set_enabled(idPtr: &str, enabled: i64, reasonPtr: &str); // TODO
    pub(crate) fn owncast_ban_ip(ipPtr: &str);
    pub(crate) fn owncast_users_register(reqPtr: &UserRegisterRequest) -> UserRegisterResult;

    // Auth
    pub(crate) fn owncast_auth_grant_session(reqPtr: &GrantSessionRequest) -> AuthResult; // TODO
    pub(crate) fn owncast_auth_end_session(); // TODO

    // Storage
    pub(crate) fn owncast_storage_upload(namePtr: &str, dataPtr: &[u8]) -> Option<UploadResult>;
    pub(crate) fn owncast_sql_exec(requestPtr: &SqlRequest) -> SqlExecResult;
    pub(crate) fn owncast_sql_query(requestPtr: &SqlRequest) -> SqlQueryResult;

    // FS
    pub(crate) fn owncast_fs_read(pathPtr: &str) -> Option<Vec<u8>>;
    pub(crate) fn owncast_fs_write(pathPtr: &str, dataPtr: &[u8]) -> FsResult;
    pub(crate) fn owncast_fs_list(dirPtr: &str) -> Json<Vec<String>>;
    pub(crate) fn owncast_fs_delete(pathPtr: &str) -> FsResult;
    pub(crate) fn owncast_fs_exists(pathPtr: &str) -> i64; // TODO

    // Fediverse
    pub(crate) fn owncast_fediverse_post(textPtr: &str) -> Option<UploadResult>;

    // Notifications
    pub(crate) fn owncast_notify_discord(textPtr: &str);
    pub(crate) fn owncast_notify_browser_push(payloadPtr: &BrowserPushPayload);
    pub(crate) fn owncast_notify_fediverse(payloadPtr: &FediversePayload);

    // KV
    pub(crate) fn owncast_kv_get(keyPtr: &str) -> Option<String>;
    pub(crate) fn owncast_kv_set(keyPtr: &str, valPtr: &str);

    // Config
    pub(crate) fn owncast_config_get<T: DeserializeOwned>(keyPtr: &str) -> Option<Json<T>>;

    // Assets
    pub(crate) fn owncast_asset_read(pathPtr: &str) -> Option<Vec<u8>>;

    // Events
    pub(crate) fn owncast_emit_event(eventTypePtr: &str, payloadPtr: &Json<impl Serialize>);

    // Actions
    pub(crate) fn owncast_add_actions(actionsPtr: &Json<Vec<ActionButton>>);
    pub(crate) fn owncast_clear_actions();

    // SSE
    pub(crate) fn owncast_sse_send(channelPtr: &str, eventPtr: &str, dataPtr: &Json<impl Serialize>);

    // Stream
    pub(crate) fn owncast_stream_current() -> Option<StreamInfo>;
    pub(crate) fn owncast_stream_broadcaster() -> Option<StreamBroadcaster>;

    // Server
    pub(crate) fn owncast_server_info() -> ServerInfo;
    pub(crate) fn owncast_server_socials() -> Json<Vec<SocialHandle>>;
    pub(crate) fn owncast_server_emotes() -> Json<Vec<Emote>>;
    pub(crate) fn owncast_server_federation() -> FederationInfo;
    pub(crate) fn owncast_server_tags() -> Json<Vec<String>>;

    // Video Config
    pub(crate) fn owncast_video_config_read() -> VideoConfig;
    pub(crate) fn owncast_video_config_write(configPtr: &VideoConfigUpdate) -> FsResult;
}

#[link(wasm_import_module = "extism:host/user")]
unsafe extern "C" {
    // Timer
    pub(crate) fn owncast_timer_set(id: i64, delayMs: i64, repeat: i64) -> i64;
    pub(crate) fn owncast_timer_clear(id: i64);
}