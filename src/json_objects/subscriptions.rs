use serde::Serialize;
use crate::json_objects::filter::Filter;
use crate::json_objects::notify::Notify;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Subscriptions {
    pub(crate) notify: Vec<Notify>,
    pub(crate) filter: Vec<Filter>
}