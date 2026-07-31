use extism_pdk::{SharedFnResult};
use crate::errors::out_of_bounds::OutOfBounds;
use crate::host::{owncast_timer_set, owncast_timer_clear};
use crate::plugin_state::PluginState;

fn do_timer(plugin_state: &mut PluginState, ms: i64, fun: fn() -> (), repeats: bool) -> SharedFnResult<i64> {
    if ms < 100 || ms > 86_400_00 {
        return Err(OutOfBounds(100, 86_400_001, ms).into())
    }
    let id = plugin_state.set_timer(fun, repeats);
    unsafe {
        owncast_timer_set(id, ms, repeats as i64)
    }
}

// Requires no permissions.
pub fn set_timeout(plugin_state: &mut PluginState, ms: i64, fun: fn() -> ()) -> SharedFnResult<i64> {
    do_timer(plugin_state, ms, fun, false)
}

// Requires no permissions.
pub fn set_interval(plugin_state: &mut PluginState, ms: i64, fun: fn() -> ()) -> SharedFnResult<i64> {
    do_timer(plugin_state, ms, fun, true)
}

// Requires no permissions.
pub fn clear(plugin_state: &mut PluginState, id: u64) -> SharedFnResult<()> {
    plugin_state.clear_timer(id);
    unsafe {
        owncast_timer_clear(id)
    }
}