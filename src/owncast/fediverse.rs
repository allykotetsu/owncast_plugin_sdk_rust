use extism_pdk::SharedFnResult;
use crate::host::owncast_fediverse_post;
use crate::json_objects::url::Url;

pub fn post(text: &str) -> SharedFnResult<Option<Url>> {
    unsafe {
        owncast_fediverse_post(text)
    }
}