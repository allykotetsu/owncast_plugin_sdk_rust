#![warn(missing_docs)]

//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com

// TODO

/// Module containing various objects, most of which are able to be either serialized to or from JSON objects. These are used for sending data to and from Extism. Examples include [`OutgoingHttpResponse`], [`FilterResult`], and [`Permission`].
pub mod json_objects;

/// Module containing objects for the [`PluginBuilder`], [`PluginState`], [`Plugin`], and other plugin-related structs. It also contains the [`define_plugin!`] macro definition that every plugin starts with.
pub mod plugin;

/// Module containing objects for commands, namely the [`CommandBuilder`], [`CommandContext`], and [`CommandDefinition`].
pub mod command;

/// Module containing miscellaneous helper functions.
pub mod helpers;

/// Module containing helper functions for accessing host functions that Owncast exposes, such as [`owncast::sql::exec`], [`owncast::events::emit`], and [`owncast::chat::send`].
pub mod owncast;

mod partial_manifest;
mod host;
mod errors;

/// [`prelude`] contains everything that is strictly necessary for getting a bare-bones plugin to compile.
pub mod prelude {
    pub use crate::define_plugin;
    pub use crate::run;
    pub use crate::errors::bad_event_type::BadEventType;
    pub use crate::helpers::clone_error;
    pub use crate::json_objects::auth_check_request::AuthCheckRequest;
    pub use crate::json_objects::auth_check_result::AuthCheckResult;
    pub use crate::json_objects::content_request::ContentRequest;
    pub use crate::json_objects::envelope::Envelope;
    pub use crate::json_objects::event_type::EventType;
    pub use crate::json_objects::filter_result::FilterResult;
    pub use crate::json_objects::incoming_http_request::IncomingHttpRequest;
    pub use crate::json_objects::manifest::Manifest;
    pub use crate::json_objects::outgoing_http_response::OutgoingHttpResponse;
    pub use crate::plugin::plugin::Plugin;
    pub use crate::plugin::plugin_builder::PluginBuilder;
    pub use crate::plugin::plugin_state::PluginState;
}

/*
TODO Test functionality

- [ ] on_fediverse
- [ ] on_fediverse_follow
- [ ] on_fediverse_like
- [ ] on_fediverse_quote
- [ ] on_fediverse_mention
- [ ] on_fediverse_reply

- [ ] owncast_auth_grant_session
- [ ] owncast_auth_end_session
- [ ] owncast_sse_send
- [ ] http
 */
