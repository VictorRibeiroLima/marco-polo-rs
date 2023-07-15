use std::io::{self, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
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
