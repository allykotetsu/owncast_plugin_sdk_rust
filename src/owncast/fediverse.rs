use extism_pdk::SharedFnResult;
use crate::host::owncast_fediverse_post;
use crate::json_objects::url::Url;

pub fn post(text: &str) -> SharedFnResult<Option<String>> {
    let url = unsafe {
        owncast_fediverse_post(text)
    };
    Ok(url?.map(|Url { url }| url))
}