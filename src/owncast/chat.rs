use extism_pdk::{Json, Memory, SharedFnResult, FromBytesOwned};
use crate::errors::mem_not_found::MemNotFound;
use crate::host::{owncast_send_chat, owncast_send_chat_action, owncast_send_chat_system, owncast_send_chat_to, owncast_chat_history, owncast_delete_message, owncast_kick_client, owncast_chat_clients};
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;

pub fn send(text: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_send_chat(text)
    }
}

pub fn send_action(text: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_send_chat_action(text)
    }
}

pub fn system(body: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_send_chat_system(body)
    }
}

pub fn send_to(client_id: i64, text: &str) -> SharedFnResult<()> {
    let text = Memory::from_bytes(text.as_bytes())?;
    unsafe {
        Ok(owncast_send_chat_to(client_id, text.offset()))
    }
}

pub fn reply_to(chat_message: impl TryInto<i64>, text: &str) -> SharedFnResult<bool> {
    let Ok(client_id) = chat_message.try_into() else {
        return Ok(false)
    };
    send_to(client_id, text).map(|_| true)
}

pub fn history(limit: Option<i64>) -> SharedFnResult<Vec<ChatMessage>> {
    let mem = unsafe {
        owncast_chat_history(limit.unwrap_or(0))
    };
    let mem = Memory::find(mem).ok_or(MemNotFound)?;
    let Json(v) = Json::<Vec<ChatMessage>>::from_bytes_owned(&mem.to_vec())?;
    Ok(v)
}

pub fn delete_message(message_id: &str) -> SharedFnResult<()> {
    unsafe {
        owncast_delete_message(message_id)
    }
}

pub fn kick(client_id: i64) -> SharedFnResult<()> {
    unsafe {
        Ok(owncast_kick_client(client_id))
    }
}

pub fn clients() -> SharedFnResult<Vec<ChatClient>> {
    unsafe {
        owncast_chat_clients().map(|Json(inner)| inner)
    }
}