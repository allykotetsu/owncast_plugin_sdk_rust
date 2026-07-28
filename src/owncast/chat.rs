use extism_pdk::Json;
use crate::errors::forbidden::Forbidden;
use crate::host::{owncast_send_chat, owncast_send_chat_action, owncast_send_chat_system, owncast_send_chat_to, owncast_chat_history, owncast_delete_message, owncast_kick_client, owncast_chat_clients};
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;

pub fn send(text: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_send_chat(text).map_err(|_| Forbidden)
    }
}

pub fn send_action(text: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_send_chat_action(text).map_err(|_| Forbidden)
    }
}

pub fn system(body: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_send_chat_system(body).map_err(|_| Forbidden)
    }
}

pub fn send_to(client_id: i64, text: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_send_chat_to(client_id, text).map_err(|_| Forbidden)
    }
}

pub fn reply_to(chat_message: impl TryInto<i64>, text: &str) -> Result<bool, Forbidden> {
    let Ok(client_id) = chat_message.try_into() else {
        return Ok(false)
    };
    send_to(client_id, text)?;
    Ok(true)
}

pub fn history(limit: Option<i64>) -> Result<Vec<ChatMessage>, Forbidden> {
    unsafe {
        owncast_chat_history(limit.unwrap_or(0)).map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}

pub fn delete_message(message_id: &str) -> Result<(), Forbidden> {
    unsafe {
        owncast_delete_message(message_id).map_err(|_| Forbidden)
    }
}

pub fn kick(client_id: i64) -> Result<(), Forbidden> {
    unsafe {
        owncast_kick_client(client_id).map_err(|_| Forbidden)
    }
}

pub fn clients() -> Result<Vec<ChatClient>, Forbidden> {
    unsafe {
        owncast_chat_clients().map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}