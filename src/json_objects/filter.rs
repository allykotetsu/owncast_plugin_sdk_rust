use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Filter {
    pub(crate) event: String,
    pub(crate) priority: u8
}