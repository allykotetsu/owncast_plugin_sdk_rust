pub mod json_objects;
mod permission;
mod input_json;
pub mod plugin_builder;
mod plugin;
mod define_plugin;
mod imports;
mod method;
mod command;
mod output_json;
mod partial_manifest;
mod errors;

pub mod prelude {
    pub use std::error::Error;
    pub use std::sync::LazyLock;
    pub use extism_pdk::FnResult;
    pub use extism_pdk::plugin_fn;
    pub use crate::define_plugin;
    pub use crate::plugin_builder::PluginBuilder;
    pub use crate::errors::BadEventType;
    pub use crate::input_json::InputJson;
    pub use crate::json_objects::auth_check_request::AuthCheckRequest;
    pub use crate::json_objects::auth_check_result::AuthCheckResult;
    pub use crate::json_objects::content_request::ContentRequest;
    pub use crate::json_objects::event::Event;
    pub use crate::json_objects::filter_result::FilterResult;
    pub use crate::json_objects::incoming_http_request::IncomingHttpRequest;
    pub use crate::json_objects::manifest::Manifest;
    pub use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
    pub use crate::output_json::OutputJson;
    pub use crate::plugin::Plugin;
}

use crate::prelude::*;

define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn std::error::Error>> {
    plugin_builder.on_chat_message(|_| {
        // owncast_send_chat(&format!("echo ${body}"));
    });
    Ok(plugin_builder)
});

/*#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::error::Error;
    use crate::command::command_builder::CommandBuilder;
    use crate::define_plugin;
    use crate::plugin_builder::PluginBuilder;
    use crate::imports::owncast_send_chat;
    use crate::json_objects::chat_message::ChatMessage;
    use crate::json_objects::stream_title_change::StreamTitleChange;
    use crate::method::Method;

    define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
        plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
            owncast_send_chat(&format!("echo ${body}"));
        });

        plugin_builder.filter_chat_message(None, |ChatMessage { body, .. }| {
            if body.contains("bad word") {
                FilterResult::Drop("No bad words allowed!".to_string())
            } else {
                FilterResult::Pass
            }
        })?;

        plugin_builder.on_stream_title_changed(|StreamTitleChange { from, to }| {
            println!("Stream name changed from {from} to {to}.");
        });

        plugin_builder.on_http_request(&[Method::GET], "/echo", &|IncomingHttpRequest { body, .. }| {
            OutgoingHttpResponse {
                status: Some(200),
                headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                body: Some(body.clone())
            }
        })?;

        plugin_builder.commands("!", false, vec![
            CommandBuilder::new("update", &|ctx| {
                ctx.reply("we've been live a while!");
            })
            .with_aliases(&["time", "livetime"])
            .with_cooldown(1000)
        ])?;

        Ok(plugin_builder)
    });

    #[test]
    fn test() {
        PLUGIN.dispatch_event(Event::StreamTitleChanged(StreamTitleChange {
            from: "Old name".to_string(),
            to: "New name".to_string(),
        }));
    }
}*/