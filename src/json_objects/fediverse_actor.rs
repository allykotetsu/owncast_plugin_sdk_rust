use serde::Deserialize;

/// An object representing a user on the fediverse.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FediverseActor {
    /// The display name of the [`FediverseActor`].
    pub name: String,

    /// The unique handle of the [`FediverseActor`], formatted as `@username@server.tld`.
    pub handle: String,

    /// The unique URL of the [`FediverseActor`]'s profile.
    pub url: Option<String>,

    /// The profile image of the [`FediverseActor`].
    pub image: Option<String>
}