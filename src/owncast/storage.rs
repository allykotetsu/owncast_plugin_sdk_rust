use extism_pdk::SharedFnResult;
use crate::host::owncast_storage_upload;
use crate::json_objects::url::Url;

pub fn upload(name: &str, data: impl Into<Vec<u8>>) -> SharedFnResult<Option<Url>> {
    let data = &data.into();
    unsafe {
        owncast_storage_upload(name, data)
    }
}