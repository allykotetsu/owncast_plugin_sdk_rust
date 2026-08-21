use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[encoding(Json)]
pub enum VideoCodec {
    Libx264,
    H264Omx,
    H264Vaapi,
    H264Qsv,
    H264Nvenc,
    H264V4l2m2m,
    H264Videotoolbox
}

impl Into<String> for &VideoCodec {
    fn into(self) -> String {
        match self {
            VideoCodec::Libx264 => "libx264",
            VideoCodec::H264Omx => "h264_omx",
            VideoCodec::H264Vaapi => "h264_vaapi",
            VideoCodec::H264Qsv => "h264_qsv",
            VideoCodec::H264Nvenc => "h264_nvenc",
            VideoCodec::H264V4l2m2m => "h264_v4l2m2m",
            VideoCodec::H264Videotoolbox => "h264_videotoolbox"
        }.to_string()
    }
}

impl PartialEq<String> for VideoCodec {
    fn eq(&self, s: &String) -> bool {
        let t: String = self.into();
        t == *s
    }
}