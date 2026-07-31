use extism_pdk::SharedFnResult;
use crate::host::owncast_storage_upload;
use crate::json_objects::upload_result::UploadResult;

pub fn upload(name: &str, data: impl Into<Vec<u8>>) -> SharedFnResult<Option<UploadResult>> {
    unsafe {
        owncast_storage_upload(name, &data.into())
    }
}