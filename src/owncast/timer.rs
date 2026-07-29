use extism_pdk::{SharedFnResult};
use crate::host::{owncast_timer_set, owncast_timer_clear};
use crate::plugin_state::PluginState;

pub fn set_timeout(plugin_state: &mut PluginState, fun: fn() -> (), ms: i64) -> SharedFnResult<i64> {
    let repeats = false;
    let id = plugin_state.set_timer(fun, repeats);
    unsafe {
        owncast_timer_set(id, ms, repeats as i64)
    }
}

pub fn set_interval(plugin_state: &mut PluginState, fun: fn() -> (), ms: i64) -> SharedFnResult<i64> {
    let repeats = true;
    let id = plugin_state.set_timer(fun, repeats);
    unsafe {
        owncast_timer_set(id, ms, repeats as i64)
    }
}

pub fn clear(plugin_state: &mut PluginState, id: i64) -> SharedFnResult<()> {
    plugin_state.clear_timer(id);
    unsafe {
        owncast_timer_clear(id)
    }
}