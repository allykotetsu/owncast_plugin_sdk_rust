use serde::Serialize;

#[derive(Serialize, Clone)]
pub(crate) struct Filter {
    pub(crate) event: String,
    pub(crate) priority: u8
}