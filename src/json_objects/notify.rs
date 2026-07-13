use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Notify {
    pub(crate) event: String
}