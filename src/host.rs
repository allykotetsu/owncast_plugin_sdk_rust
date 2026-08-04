use extism_pdk::{host_fn, Json};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::json_objects::action_button::ActionButton;
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::emote::Emote;
use crate::json_objects::federation_info::FederationInfo;
use crate::json_objects::fediverse_payload::FediversePayload;
use crate::json_objects::host_fn_result::HostFnResult;
use crate::json_objects::grant_session_request::GrantSessionRequest;
use crate::json_objects::server_info::ServerInfo;
use crate::json_objects::social_handle::SocialHandle;
use crate::json_objects::sql_exec_result::SqlExecResult;
use crate::json_objects::sql_query_result::SqlQueryResult;
use crate::json_objects::sql_request::SqlRequest;
use crate::json_objects::stream_broadcaster::StreamBroadcaster;
use crate::json_objects::stream_info::StreamInfo;
use crate::json_objects::url::Url;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;
use crate::json_objects::video_config::VideoConfig;
use crate::json_objects::video_config_update::VideoConfigUpdate;

#[host_fn]
unsafe extern "ExtismHost" {
    // Logging
    pub(crate) fn owncast_log_info(message_ptr: &str);
    pub(crate) fn owncast_log_warning(message_ptr: &str);
    pub(crate) fn owncast_log_error(message_ptr: &str);

    // Chat
    pub(crate) fn owncast_send_chat(text: &str);
    pub(crate) fn owncast_send_chat_action(text: &str);
    pub(crate) fn owncast_send_chat_system(body: &str);
    pub(crate) fn owncast_delete_message(id: &str) -> HostFnResult;
    pub(crate) fn owncast_chat_clients() -> Json<Vec<ChatClient>>;

    // Users
    pub(crate) fn owncast_users_list() -> Json<Vec<User>>;
    pub(crate) fn owncast_user_get(id: &str) -> Option<User>;
    pub(crate) fn owncast_ban_ip(ip: &str) -> HostFnResult;
    pub(crate) fn owncast_users_register(req: &UserRegisterRequest) -> UserRegisterResult;

    // Auth
    pub(crate) fn owncast_auth_grant_session(req: &GrantSessionRequest) -> HostFnResult; // TODO
    pub(crate) fn owncast_auth_end_session(); // TODO

    // Storage
    pub(crate) fn owncast_storage_upload(name: &str, data: &[u8]) -> Option<Url>;
    pub(crate) fn owncast_sql_exec(request: &SqlRequest) -> SqlExecResult;
    pub(crate) fn owncast_sql_query(request: &SqlRequest) -> SqlQueryResult;

    // FS
    pub(crate) fn owncast_fs_read(path: &str) -> Option<Vec<u8>>;
    pub(crate) fn owncast_fs_write(path: &str, data: &[u8]) -> HostFnResult;
    pub(crate) fn owncast_fs_list(dir: &str) -> Json<Vec<String>>;
    pub(crate) fn owncast_fs_delete(path: &str) -> HostFnResult;

    // Fediverse
    pub(crate) fn owncast_fediverse_post(text: &str) -> Option<Url>;

    // Notifications
    pub(crate) fn owncast_notify_discord(text: &str);
    pub(crate) fn owncast_notify_browser_push(payload: &BrowserPushPayload);
    pub(crate) fn owncast_notify_fediverse(payload: &FediversePayload);

    // KV
    pub(crate) fn owncast_kv_get(key: &str) -> Option<String>;
    pub(crate) fn owncast_kv_set(key: &str, val: &str) -> HostFnResult;

    // Config
    pub(crate) fn owncast_config_get<T: DeserializeOwned>(key: &str) -> Option<Json<T>>;

    // Assets
    pub(crate) fn owncast_asset_read(path: &str) -> Option<Vec<u8>>;

    // Events
    pub(crate) fn owncast_emit_event(event_type: &str, payload: &Json<impl Serialize>);

    // Actions
    pub(crate) fn owncast_add_actions(actions: &Json<Vec<ActionButton>>) -> HostFnResult;
    pub(crate) fn owncast_clear_actions() -> HostFnResult;

    // SSE
    pub(crate) fn owncast_sse_send(channel: &str, event: &str, data: &Json<impl Serialize>); // TODO

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
    pub(crate) fn owncast_video_config_write(config: &VideoConfigUpdate) -> HostFnResult;
}

#[link(wasm_import_module = "extism:host/user")]
unsafe extern "C" {
    // Chat
    pub(crate) fn owncast_chat_history(limit: i64) -> u64;
    pub(crate) fn owncast_send_chat_to(client_id: i64, text_ptr: u64);
    pub(crate) fn owncast_kick_client(client_id: i64) -> u64;

    // Users
    pub(crate) fn owncast_user_set_enabled(id_ptr: u64, enabled: i64, reason_ptr: u64) -> u64;

    // FS
    pub(crate) fn owncast_fs_exists(path_ptr: u64) -> i64;

    // Timer
    pub(crate) fn owncast_timer_set(id: i64, delay_ms: i64, repeat: i64) -> i64;
    pub(crate) fn owncast_timer_clear(id: i64);
}