use anyhow::anyhow;
use extism_pdk::{Json, Memory, SharedFnResult};
use crate::errors::mem_not_found::MemNotFound;
use crate::host::{owncast_send_chat, owncast_send_chat_action, owncast_send_chat_system, owncast_send_chat_to, owncast_chat_history, owncast_delete_message, owncast_kick_client, owncast_chat_clients};
use crate::json_objects::chat_client::ChatClient;
use crate::json_objects::chat_message::ChatMessage;
use crate::json_objects::action_result::ActionResult;

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
    let text = Memory::new(&text)?.offset();
    unsafe {
        Ok(owncast_send_chat_to(client_id, text))
    }
}

pub fn reply_to(chat_message: impl TryInto<i64>, text: &str) -> SharedFnResult<bool> {
    let Ok(client_id) = chat_message.try_into() else {
        return Ok(false)
    };
    send_to(client_id, text).map(|_| true)
}

pub fn history(limit: Option<i64>) -> SharedFnResult<Vec<ChatMessage>> {
    let offset = unsafe {
        owncast_chat_history(limit.unwrap_or(0))
    };
    let memory = Memory::find(offset).ok_or(MemNotFound)?;
    let Json(v) = memory.to()?;
    Ok(v)
}

pub fn delete_message(message_id: &str) -> SharedFnResult<()> {
    unsafe {
        if let Some(err) = owncast_delete_message(message_id)?.error {
            Err(anyhow!(err))
        } else {
            Ok(())
        }
    }
}

pub fn kick(client_id: i64) -> SharedFnResult<()> {
    let offset = unsafe {
        owncast_kick_client(client_id)
    };
    let memory = Memory::find(offset).ok_or(MemNotFound)?;
    let action_result: ActionResult = memory.to()?;

    if let Some(err) = action_result.error {
        Err(anyhow!(err))
    } else {
        Ok(())
    }
}

pub fn clients() -> SharedFnResult<Vec<ChatClient>> {
    unsafe {
        owncast_chat_clients().map(|Json(inner)| inner)
    }
}