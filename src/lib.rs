pub mod json_objects;
pub mod plugin_builder;
mod plugin;
mod define_plugin;
pub mod command;
mod partial_manifest;
mod state;
mod run;
pub mod owncast;
mod host;
mod errors;
mod plugin_state;
pub mod event_function;

pub mod prelude {
    pub use crate::define_plugin;
    pub use crate::define_plugin::clone;
    pub use crate::errors::bad_event_type::BadEventType;
    pub use crate::errors::dbg::Dbg;
    pub use crate::errors::missing_manifest::MissingManifest;
    pub use crate::errors::pluginless::Pluginless;
    pub use crate::plugin_builder::PluginBuilder;
    pub use crate::plugin_state::PluginState;
    pub use crate::json_objects::auth_check_request::AuthCheckRequest;
    pub use crate::json_objects::auth_check_result::AuthCheckResult;
    pub use crate::json_objects::content_request::ContentRequest;
    pub use crate::json_objects::envelope::Envelope;
    pub use crate::json_objects::event_type::EventType;
    pub use crate::json_objects::filter_result::FilterResult;
    pub use crate::json_objects::incoming_http_request::IncomingHttpRequest;
    pub use crate::json_objects::manifest::Manifest;
    pub use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
    pub use crate::plugin::Plugin;
    pub use crate::state::State;
}

use prelude::*;
define_plugin!("", |plugin_builder| {
    Ok(plugin_builder)
});