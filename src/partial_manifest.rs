use std::collections::HashMap;
use serde::Deserialize;
use crate::json_objects::action_button::ActionButton;
use crate::json_objects::admin::Admin;
use crate::json_objects::bot::Bot;
use crate::json_objects::category::Category;
use crate::json_objects::config::Config;
use crate::json_objects::extra_page_content::ExtraPageContent;
use crate::json_objects::network::Network;
use crate::json_objects::tab::Tab;
use crate::json_objects::permission::Permission;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PartialManifest {
    pub(crate) api: String,
    pub(crate) name: String,
    pub(crate) slug: Option<String>,
    pub(crate) version: String,
    pub(crate) description: String,
    pub(crate) config: Option<HashMap<String, Config>>,
    pub(crate) bot: Option<Bot>,
    pub(crate) permissions: Vec<Permission>,
    pub(crate) actions: Option<Vec<ActionButton>>,
    pub(crate) admin: Option<Admin>,
    pub(crate) network: Option<Network>,
    pub(crate) styles: Option<Vec<String>>,
    pub(crate) scripts: Option<Vec<String>>,
    pub(crate) extra_page_content: Option<ExtraPageContent>,
    pub(crate) tabs: Option<Tab>,
    pub(crate) category: Category
}