use serde::Serialize;

// TODO make sure it serializes correctly

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterResult {
    Pass,
    Modify(String),
    Drop(String)
}