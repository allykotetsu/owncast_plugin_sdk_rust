use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct UserRegisterRequest {
    pub auth_id: String,
    pub display_name: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub profile_url: Option<String>,
    pub handle: Option<String>,
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