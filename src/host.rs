use extism_pdk::{host_fn, Json};
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::user::User;
use crate::json_objects::user_register_request::UserRegisterRequest;
use crate::json_objects::user_register_result::UserRegisterResult;

#[host_fn]
extern "ExtismHost" {
    // Chat
    pub(crate) fn owncast_send_chat(textPtr: &str);
    pub(crate) fn owncast_send_chat_action(textPtr: &str);
    pub(crate) fn owncast_send_chat_system(bodyPtr: &str);
    pub(crate) fn owncast_send_chat_to(clientId: i64, textPtr: &str);
    pub(crate) fn owncast_chat_history(limit: i64) -> Json<Vec<ChatMessage>>;
    pub(crate) fn owncast_delete_message(idPtr: &str);
    pub(crate) fn owncast_kick_client(clientId: i64);
    pub(crate) fn owncast_chat_clients() -> Json<Vec<ChatClient>>;

    // Users
    pub(crate) fn owncast_users_list() -> Json<Vec<User>>;
    pub(crate) fn owncast_user_get(idPtr: &str) -> Option<User>;
    pub(crate) fn owncast_user_set_enabled(idPtr: &str, enabled: bool, reasonPtr: &str);
    pub(crate) fn owncast_ban_ip(ipPtr: &str);
    pub(crate) fn owncast_users_register(reqPtr: UserRegisterRequest) -> UserRegisterResult; // TODO

    // Auth
    /*fn owncast_auth_grant_session(reqPtr: PTR); // TODO
    fn owncast_auth_end_session();

    // Storage
    fn owncast_storage_upload(namePtr: &str, dataPtr: &[u8]) -> PTR; // TODO

    // FS


    fn owncast_timer_set(id: i64, delayMs: i64, repeat: i64) -> i64;
    fn owncast_timer_clear(id: i64);
    fn owncast_config_get(keyPtr: PTR) -> PTR;
    fn owncast_asset_read(pathPtr: PTR) -> PTR;
    fn owncast_notify_discord(textPtr: PTR);
    fn owncast_notify_browser_push(payloadPtr: PTR);
    fn owncast_notify_fediverse(payloadPtr: PTR);

    fn owncast_fs_read(pathPtr: PTR) -> PTR;
    fn owncast_fs_write(pathPtr: PTR, dataPtr: PTR) -> PTR;
    fn owncast_fs_list(dirPtr: PTR) -> PTR;
    fn owncast_fs_delete(pathPtr: PTR) -> PTR;
    fn owncast_fs_exists(pathPtr: PTR) -> i64;
    fn owncast_fediverse_post(textPtr: PTR) -> PTR;
    fn owncast_kv_get(keyPtr: PTR) -> PTR;
    fn owncast_kv_set(keyPtr: PTR, valPtr: PTR);
    fn owncast_emit_event(eventTypePtr: PTR, payloadPtr: PTR);
    fn owncast_sse_send(channelPtr: PTR, eventPtr: PTR, dataPtr: PTR);
    fn owncast_stream_current() -> PTR;
    fn owncast_server_info() -> PTR;
    fn owncast_server_socials() -> PTR;
    fn owncast_server_emotes() -> PTR;
    fn owncast_server_federation() -> PTR;
    fn owncast_stream_broadcaster() -> PTR;
    fn owncast_server_tags() -> PTR;
    fn owncast_video_config_read() -> PTR;
    fn owncast_video_config_write(configPtr: PTR) -> PTR;
    fn owncast_add_actions(actionsPtr: PTR);
    fn owncast_clear_actions();*/
}