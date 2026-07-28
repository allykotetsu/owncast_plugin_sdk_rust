use crate::errors::Forbidden;
use crate::host::owncast_storage_upload;
use crate::json_objects::url::Url;

pub fn upload(name: &str, data: &[u8]) -> Result<Option<Url>, Forbidden> {
    unsafe {
        owncast_storage_upload(name, data).map_err(|_| Forbidden)
    }
}