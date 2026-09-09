use serde::Deserialize;

#[derive(Deserialize)]
pub struct Count {
    #[serde(rename = "COUNT(*)")]
    pub count: i64
}