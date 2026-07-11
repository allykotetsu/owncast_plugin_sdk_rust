mod json_objects;
mod permissions;
mod input_json;
mod plugin_builder;
mod plugin;
mod define_plugin;
mod imports;
mod method;
mod command;
mod output_json;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::error::Error;
    use crate::command::command_builder::CommandBuilder;
    use crate::command::ctx::Ctx;
    use crate::define_plugin;
    use crate::plugin_builder::PluginBuilder;
    use crate::imports::owncast_send_chat;
    use crate::json_objects::chat_message::ChatMessage;
    use crate::method::Method;

    define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
        plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
            owncast_send_chat(format!("echo ${body}").as_str());
        });

        plugin_builder.filter_chat_message(None, |ChatMessage { body, .. }| {
            if body.contains("bad word") {
                FilterResult::Drop("No bad words allowed!".to_string())
            } else {
                FilterResult::Pass
            }
        })?;

        plugin_builder.on_http_request(&[Method::GET], "/echo", &|IncomingHttpRequest { body, .. }: IncomingHttpRequest| {
            OutgoingHttpResponse {
                status: Some(200),
                headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                body: Some(body)
            }
        })?;

        plugin_builder.on("another-plugin.something", |_payload| {
            // idk
        });

        plugin_builder.commands("!", vec![
            CommandBuilder::new("update", &|ctx: Ctx| {
                ctx.reply("we've been live a while!");
            })
            .with_aliases(&["time", "livetime"])
            .with_cooldown(1000)
        ])?;

        Ok(plugin_builder)
    });

    #[test]
    fn test() {
        println!("test");
    }
}