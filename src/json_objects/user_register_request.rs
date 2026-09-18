use extism_pdk::{ToBytes, Json};
use serde::Serialize;

/// An object representing a request to register a new [`User`].
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct UserRegisterRequest {
    /// The per-plugin unique ID tied to the [`User`]. This is different from their formal user ID, which is Owncast determined and is globally unique.
    pub auth_id: String,

    /// The display name the registered [`User`] should have.
    pub display_name: Option<String>,

    /// The scopes the registered [`User`] should have.
    pub scopes: Option<Vec<String>>,

    /// A URL to an external profile that the [`User`] owns.
    pub profile_url: Option<String>,

    /// The fediverse handle the the [`User`] owns.
    pub handle: Option<String>,

    /// Whether to display the [`User`]'s profile URL and handle publicly.
    pub public: Option<bool>,
}

impl UserRegisterRequest {
    pub fn new(auth_id: &str) -> Self {
        Self {
            auth_id: auth_id.to_string(),
            display_name: None,
            scopes: None,
            profile_url: None,
            handle: None,
            public: None,
        }
    }

    pub fn with_display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_string());
        self
    }

    pub fn with_scope(mut self, scope: &str) -> Self {
        if let Some(ref mut scopes) = self.scopes {
            scopes.push(scope.to_string());
        } else {
            self.scopes = Some(vec![scope.to_string()]);
        }
        self
    }

    pub fn with_profile_url(mut self, profile_url: &str) -> Self {
        self.profile_url = Some(profile_url.to_string());
        self
    }

    pub fn with_handle(mut self, handle: &str) -> Self {
        self.handle = Some(handle.to_string());
        self
    }

    pub fn with_is_public(mut self) -> Self {
        self.public = Some(true);
        self
    }
}

impl From<&str> for UserRegisterRequest {
    fn from(s: &str) -> UserRegisterRequest {
        UserRegisterRequest {
            auth_id: s.to_string(),
            display_name: None,
            scopes: None,
            profile_url: None,
            handle: None,
            public: None,
        }
    }
}