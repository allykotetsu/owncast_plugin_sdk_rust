use crate::errors::out_of_bounds::OutOfBounds;
use crate::host::{owncast_timer_set, owncast_timer_clear};
use crate::plugin::plugin_state::PluginState;

fn do_timer(plugin_state: &mut PluginState, ms: i64, fun: fn() -> (), repeats: bool) -> Result<Option<i64>, OutOfBounds<i64>> {
    if ms < 100 || ms > 86_400_00 {
        return Err(OutOfBounds(100, 86_400_001, ms))
    }
    let id = plugin_state.set_timer(fun, repeats);
    let res = unsafe {
        owncast_timer_set(id, ms, repeats as i64) == 1
    };
    Ok(if res { Some(id) } else { None })
}

// Requires no permissions.
/// Sets a fire-once timer in Owncast's per-plugin timer registry.
///
/// # Errors
///
/// Errors if there is an issue setting memory in Extism.
///
/// # Examples
///
/// ```
/// ```
pub fn set_timeout(plugin_state: &mut PluginState, ms: i64, fun: fn() -> ()) -> Result<Option<i64>, OutOfBounds<i64>> {
    do_timer(plugin_state, ms, fun, false)
}

/// Sets a repeating timer in Owncast's per-plugin timer registry.
///
/// # Errors
///
/// Errors if there is an issue setting memory in Extism.
///
/// # Examples
///
/// ```
/// ```
// Requires no permissions.
pub fn set_interval(plugin_state: &mut PluginState, ms: i64, fun: fn() -> ()) -> Result<Option<i64>, OutOfBounds<i64>> {
    do_timer(plugin_state, ms, fun, true)
}

// Requires no permissions.
/// Clears a given timer from Owncast's per-plugin timer registry.
///
/// # Examples
///
/// ```
/// ```
pub fn clear(plugin_state: &mut PluginState, id: i64) {
    plugin_state.clear_timer(id);
    unsafe {
        owncast_timer_clear(id)
    }
}