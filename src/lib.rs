mod json_objects;
mod permissions;
mod json;
mod plugin_builder;
mod plugin;
mod define_plugin;
mod imports;
mod method;
mod ctx;
mod command_builder;

use std::error::Error;
use crate::command_builder::CommandBuilder;
use crate::plugin_builder::PluginBuilder;
use crate::imports::owncast_send_chat;
use crate::method::Method;

define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
    plugin_builder.on_chat_message(|msg| {
        owncast_send_chat(format!("echo ${msg}").as_str());
    });

    plugin_builder.on_http_request(&[Method::GET], "/echo", &|incoming_http_request: IncomingHttpRequest| {
        OutgoingHttpResponse {
            status: Some(200),
            headers: None,
            body: Some(incoming_http_request.body)
        }
    })?;

    plugin_builder.on("another-plugin.something", |_payload| {
        // idk
    })?;

    plugin_builder.commands("!", vec![
        CommandBuilder::new("update", |ctx| {
            ctx.reply("we've been live a while!");
        })
    ])?;

    Ok(plugin_builder)
});

/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/