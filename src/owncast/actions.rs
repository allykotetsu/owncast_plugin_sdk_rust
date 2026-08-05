use extism_pdk::{Json, SharedFnResult};
use crate::host::{owncast_add_actions, owncast_clear_actions};
use crate::json_objects::action_button::ActionButton;

/// Adds action buttons to the front page. Takes either a single `ActionButton` or an array.
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
///     run!(owncast::actions::add(ActionButton::html("Click me!", "<p>Thanks for clicking on me!</p>")));
/// }
/// ```
pub fn add(actions: impl Into<Vec<ActionButton>>) -> SharedFnResult<()> {
    let actions = &Json(actions.into());
    let action_result = unsafe {
        owncast_add_actions(actions)
    };
    action_result?.try_into()
}

/// Clears runtime-set action buttons on the front page.
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
///     run!(owncast::actions::clear());
/// }
/// ```
pub fn clear() -> SharedFnResult<()> {
    let action_result = unsafe {
        owncast_clear_actions()
    };
    action_result?.try_into()
}