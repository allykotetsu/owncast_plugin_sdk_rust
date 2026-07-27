use owncast_plugin_sdk_rust::prelude::*;
use owncast_plugin_sdk_rust::json_objects::chat_message::ChatMessage;
use owncast_plugin_sdk_rust::imports::owncast;

define_plugin!(|mut plugin_builder| {
    plugin_builder.on(|ChatMessage { body, .. }| {
        // In the event that the plugin does not have chat.send permissions, owncast::chat::send will error.
        // run! is a convenience macro that logs an error in the event of that happening
        // If you want to properly handle the error, then simply remove the macro and use the base function.
        run!(owncast::chat::send(&format!("echo {body}")));
    });
    Ok(plugin_builder)
});

fn main() {}