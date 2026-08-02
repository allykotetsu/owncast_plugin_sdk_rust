/// A convenience macro for logging an error through Owncast when one arises during the use of a host function. If there is a problem logging through Owncast, then the error is logged to Extism instead.
///
/// # Examples
///
/// ```
/// use owncast_plugin_sdk_rust::{owncast, run};
///
/// fn foo() {
///     run!(owncast::chat::send("Hi everyone!"));
/// }
/// ```
#[macro_export]
macro_rules! run {
    ($func:expr) => {
        if let Err(err) = $func {
            if let Err(err) = owncast::log::error(&err.to_string()) {
                extism_pdk::error!("{err}");
            }
        }
    };
}