use serde::Serialize;

#[derive(Serialize, Clone)]
pub(crate) struct Notify {
    pub(crate) event: String
}