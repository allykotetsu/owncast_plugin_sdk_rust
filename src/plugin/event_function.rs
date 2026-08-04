use crate::plugin::plugin_state::PluginState;

/// A function pointer that takes &mut PluginState and &T and returns U.
pub type EventFunction<T, U> = fn(&mut PluginState, &T) -> U;

/// A function pointer that takes &mut PluginState and &T and returns ().
pub type EventFunctionVoid<T> = EventFunction<T, ()>;