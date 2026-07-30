use extism_pdk::SharedFnResult;
use crate::host::{owncast_video_config_read, owncast_video_config_write};
use crate::json_objects::error::Error;
use crate::json_objects::video_config::VideoConfig;
use crate::json_objects::video_config_update::VideoConfigUpdate;

pub fn read() -> SharedFnResult<VideoConfig> {
    unsafe {
        owncast_video_config_read()
    }
}

pub fn write(config: &VideoConfigUpdate) -> SharedFnResult<Error> {
    unsafe {
        owncast_video_config_write(config)
    }
}