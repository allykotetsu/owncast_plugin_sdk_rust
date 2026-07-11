/// Macro for defining one Owncast plugin. Only call this once for your project, and call it outside of function scope.
///
/// define_plugin! expects a parameter that is a `Fn(PluginBuilder<'static>) -> Result<PluginBuilder, Box<dyn Error>>` closure
///
/// Within the body of the function, call functions onto the builder for adding functionality to the plugin, and then return an Ok() wrapping the plugin builder object.
///
/// # Panics
///
/// Panics if a function called onto PluginBuilder panics.
///
/// # Examples
///
/// ```
/// define_plugin!(|mut plugin_builder: PluginBuilder<'static>| -> Result<PluginBuilder, Box<dyn Error>> {
///     plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
///         owncast_send_chat(format!("echo ${body}").as_str());
///     });
///     Ok(plugin_builder)
/// });
/// ```
#[macro_export] macro_rules! define_plugin {
    ($func:expr) => {
        use std::cell::LazyCell;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::json::Json;
        use crate::json_objects::envelope::Envelope;
        use crate::json_objects::event::Event;
        use crate::json_objects::filter_result::FilterResult;
        use crate::json_objects::incoming_http_request::IncomingHttpRequest;
        use crate::json_objects::manifest::Manifest;
        use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
        use crate::plugin::Plugin;

        const PLUGIN: LazyCell<Plugin> = LazyCell::new(|| {
            $func(PluginBuilder::new()).unwrap().into()
        });

        // Exported functions.
        #[wasm_bindgen]
        pub fn register() -> Json<Manifest> {
            Json(PLUGIN.get_manifest())
        }

        #[wasm_bindgen]
        pub fn on_event(Json(Envelope { event_type, payload }): Json<Envelope<String>>) {
            PLUGIN.on_event(event_type, payload);
        }

        #[wasm_bindgen]
        pub fn on_filter(Json(Envelope { event_type, payload }): Json<Envelope<ChatMessage>>) -> Json<FilterResult> {
            if let Event::ChatMessageReceived = event_type {
                Json(PLUGIN.on_filter(payload))
            } else {
                Json(FilterResult::Pass)
            }
        }

        #[wasm_bindgen]
        pub fn on_http_request(Json(incoming_http_request): Json<IncomingHttpRequest>) -> Json<OutgoingHttpResponse> {
            Json(PLUGIN.on_http_request(incoming_http_request))
        }

        /*#[wasm_bindgen]
        pub fn on_tab_content(_: Json<ContentRequest>) -> String {

        }

        #[wasm_bindgen]
        pub fn on_page_content(_: Json<ContentRequest>) -> String {

        }

        // Optional. Only export if exists.
        #[wasm_bindgen]
        pub fn on_page_styles() -> String {

        }

        // Optional. Only export if exists.
        #[wasm_bindgen]
        pub fn on_page_scripts() -> String {

        }

        // Optional. Only export if exists.
        #[wasm_bindgen]
        pub fn on_auth_check(_: Json<AuthCheckRequest>) -> Json<AuthCheckResult> {

        }*/
    };
}