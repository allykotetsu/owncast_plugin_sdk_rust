use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FediverseActor {
    pub name: String,
    pub handle: String,
    pub url: Option<String>,
    pub image: Option<String>
}