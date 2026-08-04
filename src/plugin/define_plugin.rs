/// Macro for defining one Owncast plugin. Only call this once for your project, and call it outside of function scope.
///
/// define_plugin! expects a parameter that is a `fn(PluginBuilder) -> FnResult<PluginBuilder>` function pointer.
///
/// Within the body of the function, call functions onto the builder for adding functionality to the plugin, and then return an Ok() wrapping the plugin builder object.
///
/// # Errors
///
/// Errors if a function called onto PluginBuilder propagates an error, or if there is a problem reading from the manifest.
///
/// # Examples
///
/// ```
/// use owncast_plugin_sdk_rust::json_objects::chat_message::ChatMessage;
/// use owncast_plugin_sdk_rust::prelude::*;
/// use owncast_plugin_sdk_rust::{owncast, helpers};
///
/// define_plugin!(|mut plugin_builder| {
///     plugin_builder.on_chat_message(|_, ChatMessage { body, .. }| {
///         run!(owncast::chat::send(&format!("echo {body}")));
///     });
///     Ok(plugin_builder)
/// });
/// ```
#[macro_export]
macro_rules! define_plugin {
    ($func:expr) => {
        const PLUGIN: std::sync::LazyLock<extism_pdk::FnResult<Plugin>> = std::sync::LazyLock::new(|| {
            let func: fn(PluginBuilder) -> extism_pdk::FnResult<PluginBuilder> = $func;
            Ok(func(PluginBuilder::new())?.try_into()?)
        });
        static mut PLUGIN_STATE: std::sync::LazyLock<PluginState> = std::sync::LazyLock::new(|| PluginState::new());

        // Exported functions.
        #[extism_pdk::plugin_fn]
        pub fn register() -> extism_pdk::FnResult<Manifest> {
            unsafe {
                PLUGIN.as_ref().map_err(clone_error)?.dispatch_init(&mut *PLUGIN_STATE);
            }
            Ok(PLUGIN.as_ref().map_err(clone_error)?.get_manifest())
        }

        #[extism_pdk::plugin_fn]
        pub fn on_event(envelope: Envelope) -> extism_pdk::FnResult<()> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_event(&mut *PLUGIN_STATE, envelope))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_filter(envelope: Envelope) -> extism_pdk::FnResult<FilterResult> {
            let Envelope::ChatMessageReceived(payload) = envelope else {
                return Err(BadEventType(EventType::ChatMessageReceived, envelope.into()).into())
            };
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_filter(&mut *PLUGIN_STATE, payload))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_http_request(incoming_http_request: IncomingHttpRequest) -> extism_pdk::FnResult<OutgoingHttpResponse> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_http_request(&mut *PLUGIN_STATE, incoming_http_request))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_tab_content(content_request: ContentRequest) -> extism_pdk::FnResult<String> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_tab_content(&mut *PLUGIN_STATE, content_request).unwrap_or(String::new()))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_page_content(content_request: ContentRequest) -> extism_pdk::FnResult<String> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_page_content(&mut *PLUGIN_STATE, content_request).unwrap_or(String::new()))
            }
        }

        // TODO is it possible to only export these functions if the plugin has the correct permissions?
        #[extism_pdk::plugin_fn]
        pub fn on_page_styles() -> extism_pdk::FnResult<String> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_page_styles(&mut *PLUGIN_STATE).unwrap_or(String::new()))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_page_scripts() -> extism_pdk::FnResult<String> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_page_scripts(&mut *PLUGIN_STATE).unwrap_or(String::new()))
            }
        }

        #[extism_pdk::plugin_fn]
        pub fn on_auth_check(auth_check_request: AuthCheckRequest) -> extism_pdk::FnResult<AuthCheckResult> {
            unsafe {
                Ok(PLUGIN.as_ref().map_err(clone_error)?.dispatch_auth_check(&mut *PLUGIN_STATE, auth_check_request).unwrap_or(AuthCheckResult::Ok))
            }
        }
    };
}