/// A convenience macro for logging an error through Extism when one arises.
#[macro_export]
macro_rules! run {
    ($func:expr) => {
        use extism_pdk::error;
        match $func {
            Err(err) => error!("{err}"),
            _ => {}
        }
    };
}