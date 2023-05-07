use std::{path::PathBuf, process::Command};

use crate::internals::cloud::traits::BucketClient;

pub async fn write_to_temp_files<BC: BucketClient + Sync>(
    bucket_client: &BC,
    temp_dir: &PathBuf,
    id: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let video_path = temp_dir.join(format!("input_{}.{}", id, "mkv"));
    let srt_path = temp_dir.join(format!("{}.{}", id, "srt"));
    let output_path = temp_dir.join(format!("output_{}.{}", id, "mkv"));

    let video_uri = format!("videos/raw/{}.{}", id, "mkv"); // for now, we only support mkv,refactor later
    let srt_uri = format!("srt_translations/{}.srt", id);

    let mut temp_file_paths = Vec::new();

    bucket_client
        .download_file_to_path(&video_uri, video_path.to_str().unwrap())
        .await?;
    bucket_client
        .download_file_to_path(&srt_uri, srt_path.to_str().unwrap())
        .await?;

    temp_file_paths.push(video_path);
    temp_file_paths.push(srt_path);
    temp_file_paths.push(output_path);

    Ok(temp_file_paths)
}

pub fn call_ffmpeg(
    video_path: &PathBuf,
    srt_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&video_path)
        .arg("-vf")
        .arg(format!("subtitles={}", &srt_path.to_str().unwrap()))
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

pub fn _read_output_file(output_path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let output_bytes = std::fs::read(&output_path)?;
    Ok(output_bytes)
}

/*
    This is just stupid.
    For some reason when i try to use the bucket_client to upload the file directly it fails with:
        HttpDispatch(HttpDispatchError { message: "Error during dispatch: connection closed before message completed" })
    For now i'm just using curl to upload the file, but i'll have to fix this later.

    ps: On a second run of the program it works on the file created on the first run, if i comment the ffmpeg call
*/
pub async fn upload_output_file<BC: BucketClient + Sync>(
    bucket_client: &BC,
    file: &PathBuf,
    video_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_uri = format!("videos/processed/{}.{}", video_id, "mkv");
    let url = bucket_client
        .create_signed_upload_url_with_uri(&output_uri, 3600)
        .await?;

    let curl_command = format!("curl -T {} '{}'", file.to_str().unwrap(), url);
    let output = Command::new("sh").arg("-c").arg(curl_command).output()?;
    if !output.status.success() {
        println!("upload output {:?}", output);
        return Err("Failed to upload file".into());
    }

    Ok(())
}

pub fn delete_temp_files(temp_file_paths: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    for path in temp_file_paths {
        std::fs::remove_file(path)?;
    }

    Ok(())
}
