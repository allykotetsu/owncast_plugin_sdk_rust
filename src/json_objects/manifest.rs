use extism_pdk::{ToBytes, Json};
use std::collections::HashMap;
use serde::Serialize;
use crate::json_objects::action_button::ActionButton;
use crate::json_objects::admin::Admin;
use crate::json_objects::bot::Bot;
use crate::json_objects::category::Category;
use crate::json_objects::command::Command;
use crate::json_objects::config::Config;
use crate::json_objects::extra_page_content::ExtraPageContent;
use crate::json_objects::network::Network;
use crate::json_objects::subscriptions::Subscriptions;
use crate::json_objects::tab::Tab;
use crate::partial_manifest::PartialManifest;
use crate::json_objects::permission::Permission;

#[derive(Serialize, Clone, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct Manifest {
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
    pub(crate) category: Category,

    pub(crate) subscriptions: Subscriptions,
    pub(crate) commands: Vec<Command>
}

impl From<(PartialManifest, Subscriptions, Vec<Command>)> for Manifest {
    fn from((partial_manifest, subscriptions, commands): (PartialManifest, Subscriptions, Vec<Command>)) -> Self {
        Self {
            api: partial_manifest.api,
            name: partial_manifest.name,
            slug: partial_manifest.slug,
            version: partial_manifest.version,
            description: partial_manifest.description,
            config: partial_manifest.config,
            bot: partial_manifest.bot,
            permissions: partial_manifest.permissions,
            actions: partial_manifest.actions,
            admin: partial_manifest.admin,
            network: partial_manifest.network,
            styles: partial_manifest.styles,
            scripts: partial_manifest.scripts,
            extra_page_content: partial_manifest.extra_page_content,
            tabs: partial_manifest.tabs,
            category: partial_manifest.category,
            subscriptions,
            commands,
        }
    }
}