#[macro_export] macro_rules! define_plugin {
    ($func:expr) => {
        use std::cell::LazyCell;
        use std::collections::HashMap;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::json::Json;
        use crate::json_objects::command::Command;
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
            let commands: Vec<Command> = PLUGIN.commands.values().map(|(_, _, _, command)| *command).collect();

            Json(Manifest {
                subscriptions: (),
                commands
            })
        }

        /*#[wasm_bindgen]
        pub fn on_event(_: Json<Envelope>) {

        }

        #[wasm_bindgen]
        pub fn on_filter(_: Json<Envelope>) -> Json<FilterResult> {

        }*/

        #[wasm_bindgen]
        pub fn on_http_request(Json(incoming_http_request): Json<IncomingHttpRequest>) -> Json<OutgoingHttpResponse> {
            if let Ok(method) = Method::try_from(&incoming_http_request.method) {
                if let Some(func) = PLUGIN.on_http_request.get(&(method, incoming_http_request.path.clone())) {
                    Json(func(incoming_http_request))
                } else {
                    Json(OutgoingHttpResponse {
                        status: None,
                        headers: None,
                        body: None
                    })
                }
            } else {
                Json(OutgoingHttpResponse {
                    status: Some(500),
                    headers: Some(HashMap::from([("Content-Type".to_string(), "text/plain".to_string())])),
                    body: Some(format!("Unable to parse request method {}.", incoming_http_request.method))
                })
            }
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