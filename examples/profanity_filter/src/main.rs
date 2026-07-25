use owncast_plugin_sdk_rust::prelude::*;
use owncast_plugin_sdk_rust::json_objects::chat_message::ChatMessage;
use owncast_plugin_sdk_rust::imports::owncast_send_chat_;

define_plugin!(|mut plugin_builder| {
    plugin_builder.on(|ChatMessage { body, .. }| {
        owncast_send_chat_(&format!("echo {body}"));
    });
    Ok(plugin_builder)
});

fn main() {}