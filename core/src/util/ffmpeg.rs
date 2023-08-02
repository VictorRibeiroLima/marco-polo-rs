use std::io::{self, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::SyncError;

use super::fs::create_temp_dir;

pub fn check() -> Result<(), io::Error> {
    let ffmpeg_output = Command::new("ffmpeg").arg("-version").output()?;

    if ffmpeg_output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "FFmpeg is not installed",
        ))
    }
}

pub fn extract_audio_from_video_to_buff(video_path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    let mut ffmpeg_output = Command::new("ffmpeg")
        .arg("-hide_banner") // Hides FFmpeg banner
        .arg("-loglevel")
        .arg("panic") // Suppresses FFmpeg output
        .arg("-i")
        .arg(&video_path)
        .arg("-vn")
        .arg("-acodec")
        .arg("libmp3lame")
        .arg("-f")
        .arg("mp3")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = ffmpeg_output
        .stdout
        .take()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to capture FFmpeg stdout"))?;

    let mut buffer = Vec::new();
    stdout.read_to_end(&mut buffer)?;

    if let Err(err) = ffmpeg_output.wait() {
        eprintln!("Error waiting for FFmpeg process: {}", err);
        return Err(err);
    }

    Ok(buffer)
}

pub fn extract_audio_from_video_to_file(
    video_path: &PathBuf,
    audio_path: &PathBuf,
) -> Result<(), io::Error> {
    let ffmpeg_output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&video_path)
        .arg("-vn")
        .arg("-acodec")
        .arg("libmp3lame")
        .arg("-q:a")
        .arg("2")
        .arg(&audio_path)
        .output()?;

    if ffmpeg_output.status.success() {
        println!("Audio extraction succeeded!");
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&ffmpeg_output.stderr);
        eprintln!("Audio extraction failed: {}", error_message);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Audio extraction failed",
        ))
    }
}

pub fn subtitle_video_to_file(
    video_path: &PathBuf,
    srt_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), SyncError> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&video_path)
        .arg("-vf")
        .arg(format!("subtitles={}", &srt_path.to_str().unwrap()))
        .arg("-c:a")
        .arg("copy")
        .arg(&output_path)
        .arg("-y")
        .output()?;

    match output.status.code() {
        Some(0) => {}
        Some(_) => {
            println!("1:{:?}", output);
            return Err("ffmpeg failed".into());
        }
        None => {
            println!("2:{:?}", output);
            return Err("ffmpeg failed".into());
        }
    }

    Ok(())
}

pub fn get_video_duration(video_path: &PathBuf) -> Result<String, io::Error> {
    let output = Command::new("ffmpeg").arg("-i").arg(&video_path).output()?;

    let output = String::from_utf8_lossy(&output.stderr); // ffmpeg will error cause because none output file is specified,this is ok

    let duration = output
        .lines()
        .find(|line| line.contains("Duration:"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "ffmpeg failed to probe video"))?
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "ffmpeg failed to probe video"))?;

    Ok(duration.to_string())
}

pub fn cut_video(
    video_path: &PathBuf,
    start_time: &str,
    end_time: &str,
) -> Result<String, io::Error> {
    let output_id = uuid::Uuid::new_v4();

    let temp_dir = create_temp_dir()?;

    let output_file = format!("{}/{}.mkv", temp_dir.to_str().unwrap(), output_id);

    let output = Command::new("ffmpeg")
        .arg("-ss")
        .arg(start_time)
        .arg("-noaccurate_seek")
        .arg("-i")
        .arg(&video_path)
        .arg("-to")
        .arg(end_time)
        .arg("-c")
        .arg("copy")
        .arg(&output_file)
        .arg("-y")
        .output()?;

    if !output.status.success() {
        println!(
            "Video cut failed. Error message: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to cut video"));
    }

    return Ok(output_file);
}
