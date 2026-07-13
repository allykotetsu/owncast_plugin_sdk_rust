use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserRegisterRequest {
    auth_id: String,
    display_name: String,
    scopes: Vec<String>
}