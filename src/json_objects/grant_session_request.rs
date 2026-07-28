use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct GrantSessionRequest {
    pub user_id: String,
    pub ttl: Option<i64>
}