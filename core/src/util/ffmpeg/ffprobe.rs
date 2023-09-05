use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::util::ffmpeg::SECONDS_TO_REDUCE;

use super::error::FfmpegError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FFProbeResult {
    pub frames: Vec<Frame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    #[serde(rename = "media_type")]
    pub media_type: String,
    #[serde(rename = "stream_index")]
    pub stream_index: i64,
    #[serde(rename = "key_frame")]
    pub key_frame: i64,
    #[serde(rename = "pkt_pts")]
    pub pkt_pts: i64,
    #[serde(rename = "pkt_pts_time")]
    pub pkt_pts_time: String,
    #[serde(rename = "pkt_dts")]
    pub pkt_dts: i64,
    #[serde(rename = "pkt_dts_time")]
    pub pkt_dts_time: String,
    #[serde(rename = "best_effort_timestamp")]
    pub best_effort_timestamp: i64,
    #[serde(rename = "best_effort_timestamp_time")]
    pub best_effort_timestamp_time: String,
    #[serde(rename = "pkt_duration")]
    pub pkt_duration: i64,
    #[serde(rename = "pkt_duration_time")]
    pub pkt_duration_time: String,
    #[serde(rename = "pkt_pos")]
    pub pkt_pos: String,
    #[serde(rename = "pkt_size")]
    pub pkt_size: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "pix_fmt")]
    pub pix_fmt: String,
    #[serde(rename = "sample_aspect_ratio")]
    pub sample_aspect_ratio: Option<String>,
    #[serde(rename = "pict_type")]
    pub pict_type: String,
    #[serde(rename = "coded_picture_number")]
    pub coded_picture_number: i64,
    #[serde(rename = "display_picture_number")]
    pub display_picture_number: i64,
    #[serde(rename = "interlaced_frame")]
    pub interlaced_frame: i64,
    #[serde(rename = "top_field_first")]
    pub top_field_first: i64,
    #[serde(rename = "repeat_pict")]
    pub repeat_pict: i64,
    #[serde(rename = "color_range")]
    pub color_range: Option<String>,
    #[serde(rename = "color_space")]
    pub color_space: Option<String>,
    #[serde(rename = "color_primaries")]
    pub color_primaries: Option<String>,
    #[serde(rename = "color_transfer")]
    pub color_transfer: Option<String>,
}

pub fn get_nearest_keyframe_in_seconds(output_file: &str) -> Result<String, FfmpegError> {
    let output = Command::new("ffprobe")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_frames")
        .arg("-read_intervals")
        .arg("%+#200")
        .arg("-skip_frame")
        .arg("nokey")
        .arg("-print_format")
        .arg("json")
        .arg("-i")
        .arg(&output_file)
        .output()?;

    if !output.status.success() {
        println!(
            "get_nearest_keyframe_in_seconds failed. Error message: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(FfmpegError::ProbeError(String::from(
            "Failed to get nearest keyframe in seconds",
        )));
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let ffprobe_result: FFProbeResult = serde_json::from_str(&output)?;
    let mut frames = ffprobe_result.frames;
    frames.reverse();
    let milliseconds_to_reduce: i64 = (SECONDS_TO_REDUCE as i64) * 1000;
    let keyframe = frames
        .iter()
        .find(|frame| frame.key_frame == 1 && frame.pkt_dts <= milliseconds_to_reduce)
        .unwrap_or(&frames[0]);

    return Ok(keyframe.pkt_dts_time.to_string());
}
