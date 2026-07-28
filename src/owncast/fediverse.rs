use crate::errors::forbidden::Forbidden;
use crate::host::owncast_fediverse_post;
use crate::json_objects::url::Url;

pub fn post(text: &str) -> Result<Option<Url>, Forbidden> {
    unsafe {
        owncast_fediverse_post(text).map_err(|_| Forbidden)
    }
}