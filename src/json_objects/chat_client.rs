use std::net::Ipv4Addr;
use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatClient {
    pub id: u64,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub connected_at: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub message_count: i64,
}

// TODO move
pub fn string_to_ipv4(ip: &Option<String>) -> Option<Ipv4Addr> {
    let ip = ip.clone()?;
    let p: Vec<&str> = ip.split(".").collect();
    Some(Ipv4Addr::new(
        p.get(0)?.parse().ok()?,
        p.get(1)?.parse().ok()?,
        p.get(2)?.parse().ok()?,
        p.get(3)?.parse().ok()?
    ))
}