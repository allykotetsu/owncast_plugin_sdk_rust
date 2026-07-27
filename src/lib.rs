pub mod json_objects;
pub mod plugin_builder;
mod plugin;
mod define_plugin;
pub mod command;
mod partial_manifest;
mod errors;
mod state;
mod run;
mod imports;

pub mod prelude {
    pub use std::error::Error;
    pub use std::sync::LazyLock;
    pub use extism_pdk::config;
    pub use extism_pdk::FnResult;
    pub use extism_pdk::error;
    pub use extism_pdk::plugin_fn;
    pub use crate::define_plugin;
    pub use crate::define_plugin::clone;
    pub use crate::plugin_builder::PluginBuilder;
    pub use crate::errors::BadEventType;
    pub use crate::errors::Dbg;
    pub use crate::errors::MissingManifest;
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

use crate::prelude::*;

define_plugin!("", (), |x| {
    Ok(x)
});