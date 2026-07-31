use extism_pdk::SharedFnResult;
use crate::host::owncast_fediverse_post;
use crate::json_objects::upload_result::UploadResult;

pub fn post(text: &str) -> SharedFnResult<Option<UploadResult>> {
    unsafe {
        owncast_fediverse_post(text)
    }
}