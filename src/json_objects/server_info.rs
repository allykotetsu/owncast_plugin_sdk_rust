use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object containing the basic Owncast server information.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ServerInfo {
    /// The name of the Owncast server.
    pub name: Option<String>,

    /// The URL of the Owncast server.
    pub url: Option<String>,

    /// The about section of the Owncast server.
    pub summary: Option<String>,

    /// The welcome message sent when someone joins chat.
    pub welcome_message: Option<String>,

    /// The Owncast version the server is running.
    pub version: Option<String>
}

impl ServerInfo {
    pub fn none() -> Self {
        ServerInfo {
            name: None,
            url: None,
            summary: None,
            welcome_message: None,
            version: None,
        }
    }
}