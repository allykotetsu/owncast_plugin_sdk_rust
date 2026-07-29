use crate::plugin_state::PluginState;

pub type EventFunction<T, U> = fn(&mut PluginState, &T) -> U;
pub type EventFunctionVoid<T> = EventFunction<T, ()>;