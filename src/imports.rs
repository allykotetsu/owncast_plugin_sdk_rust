use extism_pdk::{error, host_fn, Json};
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;

// TODO import other owncast functions

#[host_fn]
extern "ExtismHost" {
    // Chat
    fn owncast_send_chat(textPtr: &str);
    fn owncast_send_chat_action(textPtr: &str);
    fn owncast_send_chat_system(bodyPtr: &str);
    fn owncast_send_chat_to(clientId: i64, textPtr: &str);
    fn owncast_chat_history(limit: i64) -> Json<Vec<ChatMessage>>;
    fn owncast_delete_message(idPtr: &str);
    fn owncast_kick_client(clientId: i64);
    fn owncast_chat_clients() -> Json<Vec<ChatClient>>;

    // Users
    /*fn owncast_users_list() -> Vec<User>;
    fn owncast_user_get(idPtr: &str) -> Option<User>;
    fn owncast_user_set_enabled(idPtr: &str, enabled: bool, reasonPtr: Option<String>);
    fn owncast_ban_ip(ipPtr: &str);
    fn owncast_users_register(reqPtr: PTR) -> PTR; // TODO

    // Auth
    fn owncast_auth_grant_session(reqPtr: PTR); // TODO
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

pub fn owncast_send_chat_(i: &str) {
    unsafe {
        match owncast_send_chat(i) {
            Err(err) => error!("{err}"),
            _ => ()
        }
    }
}

pub fn owncast_send_chat_to_(i: i64, j: &str) {
    unsafe {
        match owncast_send_chat_to(i, j) {
            Err(err) => error!("{err}"),
            _ => ()
        }
    }
}

/*macro_rules! permitted {
    ($permission:expr, $func:expr) => {
        pub fn $func() -> Result<, Forbidden> {
            unsafe {
                $func
            }
        }
    };
}

permitted!(Permission::ChatSend, owncast_send_chat_action);*/

// #[permitted(Permission::ChatSend)]