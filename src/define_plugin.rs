/// Macro for defining one Owncast plugin. Only call this once for your project, and call it outside of function scope.
///
/// define_plugin! expects a parameter that is a `fn(PluginBuilder<'static>) -> Result<PluginBuilder, Box<dyn Error>>` function pointer.
///
/// Within the body of the function, call functions onto the builder for adding functionality to the plugin, and then return an Ok() wrapping the plugin builder object.
///
/// # Panics
///
/// Panics if a function called onto PluginBuilder results in an error.
///
/// # Examples
///
/// ```
/// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
///     plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
///         owncast_send_chat(&format!("echo {body}"));
///     });
///     Ok(plugin_builder)
/// });
/// ```
#[macro_export]
macro_rules! define_plugin {
    ($func:expr) => {
        use std::collections::HashMap;
        use std::sync::LazyLock;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::input_json::InputJson;
        use crate::json_objects::auth_check_request::AuthCheckRequest;
        use crate::json_objects::auth_check_result::AuthCheckResult;
        use crate::json_objects::content_request::ContentRequest;
        use crate::json_objects::event::Event;
        use crate::json_objects::filter_result::FilterResult;
        use crate::json_objects::incoming_http_request::IncomingHttpRequest;
        use crate::json_objects::manifest::Manifest;
        use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
        use crate::output_json::OutputJson;
        use crate::plugin::Plugin;

        const PLUGIN: LazyLock<Plugin> = LazyLock::new(|| {
            $func(PluginBuilder::new().unwrap()).unwrap().into()
        });

        // Exported functions.
        #[wasm_bindgen]
        pub fn register() -> OutputJson<Manifest> {
            OutputJson(PLUGIN.get_manifest())
        }

        #[wasm_bindgen]
        pub fn on_event(InputJson(event): InputJson<Event>) {
            match event {
                Ok(event) => PLUGIN.dispatch_event(event),
                Err(err) => println!("{err}")
            }
        }

        #[wasm_bindgen]
        pub fn on_filter(InputJson(event): InputJson<Event>) -> OutputJson<FilterResult> {
            match event {
                Ok(event) => {
                    if let Event::ChatMessageReceived(payload) = event {
                        OutputJson(PLUGIN.dispatch_filter(payload))
                    } else {
                        let event: String = event.into();
                        println!("Expected filter for \"chat.message.received\", got {event}");
                        OutputJson(FilterResult::Pass)
                    }
                }
                Err(err) => {
                    println!("{err}");
                    OutputJson(FilterResult::Pass)
                }
            }
        }

        #[wasm_bindgen]
        pub fn on_http_request(InputJson(incoming_http_request): InputJson<IncomingHttpRequest>) -> OutputJson<OutgoingHttpResponse> {
            match incoming_http_request {
                Ok(incoming_http_request) => OutputJson(PLUGIN.dispatch_http_request(incoming_http_request)),
                Err(err) => {
                    println!("{err}");
                    OutputJson(OutgoingHttpResponse {
                        status: Some(400),
                        headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                        body: Some("Couldn't deserialize incoming HTTP request.".to_string())
                    })
                }
            }
        }

        #[wasm_bindgen]
        pub fn on_tab_content(InputJson(content_request): InputJson<ContentRequest>) -> String {
            match content_request {
                Ok(content_request) => PLUGIN.dispatch_tab_content(content_request).unwrap_or(String::new()),
                Err(err) => {
                    println!("{err}");
                    String::new()
                }
            }
        }

        #[wasm_bindgen]
        pub fn on_page_content(InputJson(content_request): InputJson<ContentRequest>) -> String {
            match content_request {
                Ok(content_request) => PLUGIN.dispatch_page_content(content_request).unwrap_or(String::new()),
                Err(err) => {
                    println!("{err}");
                    String::new()
                }
            }
        }

        // TODO is it possible to only export these functions if the plugin has the correct permissions?
        #[wasm_bindgen]
        pub fn on_page_styles() -> String {
            PLUGIN.dispatch_page_styles().unwrap_or(String::new())
        }

        #[wasm_bindgen]
        pub fn on_page_scripts() -> String {
            PLUGIN.dispatch_page_scripts().unwrap_or(String::new())
        }

        #[wasm_bindgen]
        pub fn on_auth_check(InputJson(auth_check_request): InputJson<AuthCheckRequest>) -> OutputJson<AuthCheckResult> {
            match auth_check_request {
                Ok(auth_check_request) => OutputJson(PLUGIN.dispatch_auth_check(auth_check_request).unwrap_or(AuthCheckResult::Ok)),
                Err(err) => {
                    println!("{err}");
                    OutputJson(AuthCheckResult::Ok)
                }
            }
        }
    };
}