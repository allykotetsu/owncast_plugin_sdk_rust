/// Macro for defining one Owncast plugin. Only call this once for your project, and call it outside of function scope.
///
/// define_plugin! expects a parameter that is a `fn(PluginBuilder<'static>) -> Result<PluginBuilder, Box<dyn Error>>` function pointer.
///
/// Within the body of the function, call functions onto the builder for adding functionality to the plugin, and then return an Ok() wrapping the plugin builder object.
///
/// # Panics
///
/// Panics if a function called onto PluginBuilder propagates an error, or if there is a problem reading from the manifest.
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
        const PLUGIN: LazyLock<Plugin> = LazyLock::new(|| {
            $func(PluginBuilder::new().unwrap()).unwrap().into()
        });

        // Exported functions.
        #[plugin_fn]
        pub fn register() -> FnResult<Manifest> {
            Ok(PLUGIN.get_manifest())
        }

        #[plugin_fn]
        pub fn on_event(event: Event) -> FnResult<()> {
            Ok(PLUGIN.dispatch_event(event))
        }

        #[plugin_fn]
        pub fn on_filter(event: Event) -> FnResult<FilterResult> {
            let payload = if let Event::ChatMessageReceived(payload) = event {
                Ok(payload)
            } else {
                let event: String = event.into();
                Err(BadEventType(format!("Expected filter for \"chat.message.received\", got {event}")))
            }?;

            Ok(PLUGIN.dispatch_filter(payload))
        }

        #[plugin_fn]
        pub fn on_http_request(incoming_http_request: IncomingHttpRequest) -> FnResult<OutgoingHttpResponse> {
            Ok(PLUGIN.dispatch_http_request(incoming_http_request))
        }

        #[plugin_fn]
        pub fn on_tab_content(content_request: ContentRequest) -> FnResult<String> {
            Ok(PLUGIN.dispatch_tab_content(content_request).unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_page_content(content_request: ContentRequest) -> FnResult<String> {
            Ok(PLUGIN.dispatch_page_content(content_request).unwrap_or(String::new()))
        }

        // TODO is it possible to only export these functions if the plugin has the correct permissions?
        #[plugin_fn]
        pub fn on_page_styles() -> FnResult<String> {
            Ok(PLUGIN.dispatch_page_styles().unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_page_scripts() -> FnResult<String> {
            Ok(PLUGIN.dispatch_page_scripts().unwrap_or(String::new()))
        }

        #[plugin_fn]
        pub fn on_auth_check(auth_check_request: AuthCheckRequest) -> FnResult<AuthCheckResult> {
            Ok(PLUGIN.dispatch_auth_check(auth_check_request).unwrap_or(AuthCheckResult::Ok))
        }
    };
}