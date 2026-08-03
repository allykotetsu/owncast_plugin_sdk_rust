use extism_pdk::{Json, SharedFnResult};
use serde::de::DeserializeOwned;
use crate::host::owncast_config_get;

// Does not require permissions.
/// Gets a value from the manifest-described config page with the specified key.
///
/// # Errors
///
/// Errors if there is an issue setting memory in Extism.
///
/// # Examples
///
/// ```
/// use std::error::Error;
/// use owncast_plugin_sdk_rust::json_objects::action_button::ActionButton;
/// use owncast_plugin_sdk_rust::{owncast, helpers};
///
/// fn foo() {
///     if let Ok(Some(foo)) = owncast::config::get("foo") {
///         run!(owncast::chat::send(foo));
///     }
/// }
/// ```
pub fn get<T: DeserializeOwned>(key: &str) -> SharedFnResult<Option<T>> {
    unsafe {
        Ok(match owncast_config_get(key)? {
            Some(Json(value)) => value,
            None => None
        })
    }
}