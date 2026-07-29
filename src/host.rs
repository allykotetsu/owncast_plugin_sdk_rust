use extism_pdk::{host_fn, Json};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::json_objects::action_button::ActionButton;
use crate::json_objects::browser_push_payload::BrowserPushPayload;
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::error::Error;
use crate::json_objects::fediverse_payload::FediversePayload;
use crate::json_objects::fs_result::FsResult;
use crate::json_objects::grant_session_request::GrantSessionRequest;
use crate::json_objects::url::Url;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;

#[host_fn]
extern "ExtismHost" {
    // Chat
    pub(crate) fn owncast_send_chat(textPtr: &str);
    pub(crate) fn owncast_send_chat_action(textPtr: &str);
    pub(crate) fn owncast_send_chat_system(bodyPtr: &str);
    pub(crate) fn owncast_send_chat_to(clientId: i64, textPtr: &str); // TODO haven't verified as working.
    pub(crate) fn owncast_chat_history(limit: i64) -> Json<Vec<ChatMessage>>;
    pub(crate) fn owncast_delete_message(idPtr: &str);
    pub(crate) fn owncast_kick_client(clientId: i64);
    pub(crate) fn owncast_chat_clients() -> Json<Vec<ChatClient>>; // TODO haven't verified as working.

    // Users
    pub(crate) fn owncast_users_list() -> Json<Vec<User>>; // TODO haven't verified as working.
    pub(crate) fn owncast_user_get(idPtr: &str) -> Option<User>; // TODO haven't verified as working.
    pub(crate) fn owncast_user_set_enabled(idPtr: &str, enabled: bool, reasonPtr: &str); // TODO haven't verified as working.
    pub(crate) fn owncast_ban_ip(ipPtr: &str); // TODO haven't verified as working.
    pub(crate) fn owncast_users_register(reqPtr: &UserRegisterRequest) -> UserRegisterResult; // TODO haven't verified as working.

    // Auth
    pub(crate) fn owncast_auth_grant_session(reqPtr: &GrantSessionRequest) -> Error; // TODO haven't verified as working.
    pub(crate) fn owncast_auth_end_session(); // TODO haven't verified as working.

    // Storage
    pub(crate) fn owncast_storage_upload(namePtr: &str, dataPtr: &[u8]) -> Option<Url>; // TODO haven't verified as working.

    // FS
    pub(crate) fn owncast_fs_read(pathPtr: &str) -> Option<Vec<u8>>; // TODO haven't verified as working.
    pub(crate) fn owncast_fs_write(pathPtr: &str, dataPtr: &[u8]) -> FsResult; // TODO haven't verified as working.
    pub(crate) fn owncast_fs_list(dirPtr: &str) -> Json<Vec<String>>; // TODO haven't verified as working.
    pub(crate) fn owncast_fs_delete(pathPtr: &str) -> FsResult; // TODO haven't verified as working.
    pub(crate) fn owncast_fs_exists(pathPtr: &str) -> i64; // TODO haven't verified as working.

    // Fediverse
    pub(crate) fn owncast_fediverse_post(textPtr: &str) -> Option<Url>; // TODO haven't verified as working.

    // Notifications
    pub(crate) fn owncast_notify_discord(textPtr: &str); // TODO haven't verified as working.
    pub(crate) fn owncast_notify_browser_push(payloadPtr: &BrowserPushPayload); // TODO haven't verified as working.
    pub(crate) fn owncast_notify_fediverse(payloadPtr: &FediversePayload); // TODO haven't verified as working.

    // KV
    pub(crate) fn owncast_kv_get(keyPtr: &str) -> Option<String>; // TODO haven't verified as working.
    pub(crate) fn owncast_kv_set(keyPtr: &str, valPtr: &str); // TODO haven't verified as working.

    // Config
    pub(crate) fn owncast_config_get<T: DeserializeOwned>(keyPtr: &str) -> Option<Json<T>>; // TODO haven't verified as working.

    // Assets
    pub(crate) fn owncast_asset_read(pathPtr: &str) -> Option<Vec<u8>>; // TODO haven't verified as working.

    // Events
    pub(crate) fn owncast_emit_event(eventTypePtr: &str, payloadPtr: Json<impl Serialize>); // TODO haven't verified as working.

    // Actions
    pub(crate) fn owncast_add_actions(actionsPtr: Json<Vec<ActionButton>>); // TODO haven't verified as working.
    pub(crate) fn owncast_clear_actions(); // TODO haven't verified as working.

    /*pub(crate) fn owncast_timer_set(id: i64, delayMs: i64, repeat: i64) -> i64;
    pub(crate) fn owncast_timer_clear(id: i64);
    pub(crate) fn owncast_sse_send(channelPtr: PTR, eventPtr: PTR, dataPtr: PTR);
    pub(crate) fn owncast_stream_current() -> PTR;
    pub(crate) fn owncast_server_info() -> PTR;
    pub(crate) fn owncast_server_socials() -> PTR;
    pub(crate) fn owncast_server_emotes() -> PTR;
    pub(crate) fn owncast_server_federation() -> PTR;
    pub(crate) fn owncast_stream_broadcaster() -> PTR;
    pub(crate) fn owncast_server_tags() -> PTR;
    pub(crate) fn owncast_video_config_read() -> PTR;
    pub(crate) fn owncast_video_config_write(configPtr: PTR) -> PTR;*/
}