use std::net::Ipv4Addr;
use anyhow::Error;
use extism_pdk::WithReturnCode;

/// A convenience macro for logging an error through Owncast when one arises during the use of a host function. If there is a problem logging through Owncast, then the error is logged to Extism instead.
///
/// * `$func` - What fallible function to run.
///
/// # Examples
///
/// ```
/// use owncast_plugin_sdk_rust::{owncast, helpers};
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

/// Clone a WithReturnCode<Error> error.
pub fn clone_error(WithReturnCode(t, u): &WithReturnCode<Error>) -> WithReturnCode<Error> {
    WithReturnCode(anyhow::anyhow!("{}", t), *u)
}

/// Attempts to convert a string into an IPv4 Address.
///
/// * `ip` - What fallible function to run.
///
/// # Examples
///
/// ```
/// use owncast_plugin_sdk_rust::{owncast, helpers};
///
/// fn foo() {
///     run!(owncast::chat::send("Hi everyone!"));
/// }
/// ```
pub fn string_to_ipv4(ip: &Option<String>) -> Option<Ipv4Addr> {
    let ip = ip.clone()?;
    let p: Vec<&str> = ip.split(".").collect();
    Some(Ipv4Addr::new(
        p.get(0)?.parse().ok()?,
        p.get(1)?.parse().ok()?,
        p.get(2)?.parse().ok()?,
        p.get(3)?.parse().ok()?
    ))
}