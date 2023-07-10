use std::{path::PathBuf, process::Command};

use crate::internals::cloud::traits::BucketClient;

pub async fn write_to_temp_files<BC: BucketClient + Sync>(
    bucket_client: &BC,
    temp_dir: &PathBuf,
    id: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Sync + Send>> {
    let video_path = temp_dir.join(format!("input_{}.{}", id, "mkv"));
    let srt_path = temp_dir.join(format!("{}.{}", id, "srt"));
    let output_path = temp_dir.join(format!("output_{}.{}", id, "mkv"));

    let video_uri = format!("videos/raw/{}.{}", id, "mkv"); // for now, we only support mkv,refactor later
    let srt_uri = format!("srt_translations/{}.srt", id);

    let mut temp_file_paths = Vec::new();

    bucket_client
        .download_file_to_path(&video_uri, video_path.to_str().unwrap())
        .await?;
    let result = bucket_client
        .download_file_to_path(&srt_uri, srt_path.to_str().unwrap())
        .await;

    match result {
        Ok(_) => {}
        Err(e) => {
            temp_file_paths.push(video_path);
            delete_temp_files(temp_file_paths)?;
            return Err(e);
        }
    }

    temp_file_paths.push(video_path);
    temp_file_paths.push(srt_path);
    temp_file_paths.push(output_path);

    Ok(temp_file_paths)
}

pub fn call_ffmpeg(
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

pub fn _read_output_file(
    output_path: &PathBuf,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>> {
    let output_bytes = std::fs::read(&output_path)?;
    Ok(output_bytes)
}

pub async fn upload_output_file<BC: BucketClient + Sync>(
    bucket_client: &BC,
    file: &PathBuf,
    video_id: &str,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let output_uri = format!("videos/processed/{}.{}", video_id, "mkv");
    bucket_client
        .upload_file_from_path(&output_uri, file.to_str().unwrap())
        .await?;

    Ok(())
}

pub fn delete_temp_files(
    temp_file_paths: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    for path in temp_file_paths {
        std::fs::remove_file(path)?;
    }

    Ok(())
}
