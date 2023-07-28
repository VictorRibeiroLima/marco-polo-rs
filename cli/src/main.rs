use args::Args;
use clap::Parser;
use marco_polo_rs_core::{
    internals::transcriber::{
        assembly_ai::AssemblyAiClient,
        traits::{Sentence, TranscriberClient},
    },
    util::ffmpeg,
    SyncError,
};
use srt::{get_srt_string, write_srt_file};

use std::env;
mod args;
mod keys;
mod srt;

const ASSEMBLY_AI_BASE_URL: &str = "https://api.assemblyai.com/v2";
const DEEPL_BASE_URL: &str = "https://api.deepl.com/v2/translate";
const GOOGLE_TRANSLATE_API_V2: &str = "https://translation.googleapis.com/language/translate/v2";

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match ffmpeg::check() {
        Ok(_) => {}
        Err(_) => {
            let err = r#"FFmpeg is not installed.
Please install FFmpeg and try again.
On macOS, you can install FFmpeg with Homebrew:
    brew install ffmpeg
On Ubuntu, you can install FFmpeg with apt:
    sudo apt install ffmpeg
On Windows, you can install FFmpeg by downloading a build from the official website:
    https://ffmpeg.org/download.html"#;
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }

    setup_env(&args);

    let assembly_ai_client =
        marco_polo_rs_core::internals::transcriber::assembly_ai::AssemblyAiClient::new();

    let sentences = match get_sentences(assembly_ai_client, &args).await {
        Ok(sentences) => sentences,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let srt_file_string = match get_srt_string(sentences, &args).await {
        Ok(srt_file_string) => srt_file_string,
        Err(_) => {
            std::process::exit(1);
        }
    };

    let srt_path_string = match args.srt_only {
        false => "./output.srt".to_string(),
        true => {
            let path = std::path::PathBuf::from(&args.output);
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let path_str = format!("{}.srt", file_stem);
            path_str
        }
    };

    match write_srt_file(&srt_path_string, srt_file_string) {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }

    if args.srt_only {
        println!("Srt file written to {}", srt_path_string);
        std::process::exit(0);
    } else {
        match write_subtitles_to_video(&args).await {
            Ok(_) => {
                println!("Video written to {}", args.output);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn write_subtitles_to_video(args: &Args) -> Result<(), SyncError> {
    let input_path = std::path::PathBuf::from(&args.input);
    let srt_path = std::path::PathBuf::from("./output.srt");
    let output_path = std::path::PathBuf::from(&args.output);

    println!("Writing subtitles to video...");
    println!("This may take a while...");

    ffmpeg::subtitle_video_to_file(&input_path, &srt_path, &output_path)?;

    std::fs::remove_file("./output.srt")?;
    Ok(())
}

async fn get_sentences(
    assembly_ai_client: AssemblyAiClient,
    args: &Args,
) -> Result<Vec<Sentence>, SyncError> {
    println!("Extracting audio from video...");
    println!("Sending audio to AssemblyAI...");
    let transcription_id = assembly_ai_client.transcribe_from_file(&args.input).await?;

    println!("Waiting for transcription to complete...");
    println!("This may take a while...");
    assembly_ai_client.pool(&transcription_id).await?;

    println!("Transcription complete!");

    let sentences = assembly_ai_client
        .get_transcription_sentences(&transcription_id)
        .await?;
    Ok(sentences)
}

fn setup_env(args: &Args) {
    env::set_var("API_URL", "");

    let keys = match keys::Keys::new(&args.keys) {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    if args.translation_service == "deepl" {
        env::set_var("DEEPL_BASE_URL", DEEPL_BASE_URL);
        env::set_var(
            "DEEPL_API_KEY",
            keys.deepl.expect(
                "deepl to be set on the 'keys' file when 'deepl' is set as the translation service",
            ),
        );
    } else if args.translation_service == "google" {
        env::set_var("GOOGLE_TRANSLATE_API_BASE_URL", GOOGLE_TRANSLATE_API_V2);
        env::set_var(
            "GOOGLE_TRANSLATE_API_KEY",
            keys.google.expect(
                "google to be set on the 'keys' file when 'google' is set as the translation service",
            ),
        );
    } else {
        eprintln!(
            "Translation service '{}' not supported",
            args.translation_service
        );
        std::process::exit(1);
    }

    env::set_var("ASSEMBLY_AI_WEBHOOK_ENDPOINT", "");
    env::set_var("ASSEMBLY_AI_WEBHOOK_TOKEN", "");
    env::set_var("ASSEMBLY_AI_BASE_URL", ASSEMBLY_AI_BASE_URL);
    env::set_var("ASSEMBLY_AI_API_KEY", "test");
    env::set_var("ASSEMBLY_AI_API_KEY", keys.assembly_ai);
}
