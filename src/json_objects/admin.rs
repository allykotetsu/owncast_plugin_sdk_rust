use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::json_objects::admin_page::AdminPage;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Admin {
    pub(crate) pages: HashMap<String, AdminPage>
}