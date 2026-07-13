use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FediverseActor {
    pub(crate) name: String,
    pub(crate) handle: String,
    pub(crate) url: Option<String>,
    pub(crate) image: Option<String>
}