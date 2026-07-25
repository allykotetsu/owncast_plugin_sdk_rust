use extism_pdk::{Error, WithReturnCode};

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
/// define_plugin!(|mut plugin_builder| {
///     plugin_builder.on_chat_message(|ChatMessage { body, .. }| {
///         owncast_send_chat(&format!("echo {body}"));
///     });
///     Ok(plugin_builder)
/// });
/// ```
#[macro_export]
macro_rules! define_plugin {
    ($func:expr) => {
        const PLUGIN: LazyLock<FnResult<Plugin>> = LazyLock::new(|| {
            let func: fn(PluginBuilder) -> FnResult<PluginBuilder> = $func;
            Ok(func(PluginBuilder::new()?)?.into())
        });

        // Exported functions.
        #[plugin_fn]
        pub fn register() -> FnResult<Manifest> {
            Ok(PLUGIN.as_ref().map_err(clone)?.get_manifest())
        }

        #[plugin_fn]
        pub fn on_event(envelope: Envelope) -> FnResult<()> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_event(envelope))
        }

        #[plugin_fn]
        pub fn on_filter(envelope: Envelope) -> FnResult<FilterResult> {
            let Envelope::ChatMessageReceived(payload) = envelope else {
                return Err(BadEventType(EventType::ChatMessageReceived, envelope.into()).into())
            };
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_filter(payload))
        }

        #[plugin_fn]
        pub fn on_http_request(incoming_http_request: IncomingHttpRequest) -> FnResult<OutgoingHttpResponse> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_http_request(incoming_http_request))
        }

        #[plugin_fn]
        pub fn on_tab_content(content_request: ContentRequest) -> FnResult<String> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_tab_content(content_request).unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_page_content(content_request: ContentRequest) -> FnResult<String> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_page_content(content_request).unwrap_or(String::new()))
        }

        // TODO is it possible to only export these functions if the plugin has the correct permissions?
        #[plugin_fn]
        pub fn on_page_styles() -> FnResult<String> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_page_styles().unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_page_scripts() -> FnResult<String> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_page_scripts().unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_auth_check(auth_check_request: AuthCheckRequest) -> FnResult<AuthCheckResult> {
            Ok(PLUGIN.as_ref().map_err(clone)?.dispatch_auth_check(auth_check_request).unwrap_or(AuthCheckResult::Ok))
        }
    };
}

pub fn clone(WithReturnCode(t, u): &WithReturnCode<Error>) -> WithReturnCode<Error> {
    WithReturnCode(anyhow::anyhow!("{}", t), *u)
}
