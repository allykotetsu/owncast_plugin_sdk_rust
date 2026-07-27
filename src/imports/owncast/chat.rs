use extism_pdk::Json;
use crate::errors::Forbidden;
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;

pub fn send(i: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_send_chat(i).map_err(|_| Forbidden)
    }
}

pub fn send_action(i: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_send_chat_action(i).map_err(|_| Forbidden)
    }
}

pub fn system(i: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_send_chat_system(i).map_err(|_| Forbidden)
    }
}

pub fn history(i: Option<i64>) -> Result<Vec<ChatMessage>, Forbidden> {
    unsafe {
        crate::imports::host::owncast_chat_history(i.unwrap_or(0)).map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}

pub fn delete_message(i: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_delete_message(i).map_err(|_| Forbidden)
    }
}

pub fn kick(i: i64) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_kick_client(i).map_err(|_| Forbidden)
    }
}

pub fn send_to(i: i64, j: &str) -> Result<(), Forbidden> {
    unsafe {
        crate::imports::host::owncast_send_chat_to(i, j).map_err(|_| Forbidden)
    }
}

// TODO implement reply_to?

pub fn clients() -> Result<Vec<ChatClient>, Forbidden> {
    unsafe {
        crate::imports::host::owncast_chat_clients().map(|Json(inner)| inner).map_err(|_| Forbidden)
    }
}