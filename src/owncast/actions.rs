use extism_pdk::{Json, SharedFnResult};
use crate::host::{owncast_add_actions, owncast_clear_actions};
use crate::json_objects::action_button::ActionButton;

pub fn actions(actions: &Vec<ActionButton>) -> SharedFnResult<()> {
    unsafe {
        owncast_add_actions(&Json(actions.clone()))
    }
}

pub fn clear() -> SharedFnResult<()> {
    unsafe {
        owncast_clear_actions()
    }
}