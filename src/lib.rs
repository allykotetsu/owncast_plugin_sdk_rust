mod objects;
mod permissions;
mod json;
mod plugin_builder;
mod plugin;
mod owncast_plugin;
mod imports;
mod method;

use crate::plugin_builder::PluginBuilder;
use crate::imports::owncast_send_chat;
use crate::method::Method;

define_plugin!(|mut plugin_builder: PluginBuilder| -> PluginBuilder {
    plugin_builder.on_chat_message(|msg| {
        owncast_send_chat(format!("echo ${msg}").as_str());
    });

    plugin_builder.on_http_request(Method::GET, "/endpoint", |incoming_http_request: IncomingHttpRequest| {
        OutgoingHttpResponse {
            status: Some(200),
            headers: None,
            body: Some("message".to_string())
        }
    }).unwrap();

    plugin_builder
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