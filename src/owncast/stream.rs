use extism_pdk::{SharedFnResult};
use crate::host::{owncast_stream_current, owncast_stream_broadcaster};
use crate::json_objects::stream_broadcaster::StreamBroadcaster;
use crate::json_objects::stream_info::StreamInfo;

pub fn current() -> SharedFnResult<StreamInfo> {
    unsafe {
        Ok(owncast_stream_current()?.unwrap_or(StreamInfo::offline()))
    }
}

pub fn broadcaster() -> SharedFnResult<Option<StreamBroadcaster>> {
    unsafe {
        Ok(owncast_stream_broadcaster()?)
    }
}