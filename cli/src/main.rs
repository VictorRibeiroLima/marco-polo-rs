use args::Args;
use clap::Parser;
use marco_polo_rs_core::{
    internals::{
        transcriber::{
            assembly_ai::AssemblyAiClient,
            traits::{Sentence, TranscriberClient},
        },
        translator::{deepl::DeeplClient, traits::TranslatorClient},
    },
    util::{ffmpeg, srt},
    SyncError,
};

use futures::future::join_all;
use std::{env, fs::File, io::Write};
mod args;
mod keys;

const ASSEMBLY_AI_BASE_URL: &str = "https://api.assemblyai.com/v2";
const DEEPL_BASE_URL: &str = "https://api-free.deepl.com/v2/translate";

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

    let deepl_client = marco_polo_rs_core::internals::translator::deepl::DeeplClient::new();

    let sentences = match get_sentences(assembly_ai_client, &args).await {
        Ok(sentences) => sentences,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let srt_file_string = match get_srt_file_string(sentences, deepl_client).await {
        Ok(srt_file_string) => srt_file_string,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match File::create("./output.srt") {
        Ok(mut file) => match file.write_all(srt_file_string.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Unable to write to output.srt: {}", e);
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!(
                "Unable to create output.srt pls put it in the same directory as the executable"
            );
            std::process::exit(1);
        }
    }

    let input_path = std::path::PathBuf::from(&args.input);
    let srt_path = std::path::PathBuf::from("./output.srt");
    let output_path = std::path::PathBuf::from(&args.output);

    println!("Writing subtitles to video...");
    println!("This may take a while...");

    match ffmpeg::subtitle_video_to_file(&input_path, &srt_path, &output_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

    match std::fs::remove_file("./output.srt") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
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

async fn get_srt_file_string(
    sentences: Vec<Sentence>,
    deepl_client: DeeplClient,
) -> Result<String, SyncError> {
    let mut translation_futures = vec![];

    for sen in sentences {
        let translation = get_translated_sentence(sen, &deepl_client);
        translation_futures.push(translation);
    }

    println!("Translating sentences...");
    let resp = join_all(translation_futures).await;

    let mut translated_sentences = vec![];
    for sentence in resp {
        translated_sentences.push(sentence?);
    }

    println!("Creating new srt file...");
    let new_srt_buffer = srt::create_based_on_sentences(translated_sentences);
    Ok(new_srt_buffer)
}

async fn get_translated_sentence(
    payload: Sentence,
    deepl_client: &DeeplClient,
) -> Result<Sentence, Box<dyn std::error::Error + Sync + Send>> {
    let translation = deepl_client.translate_sentence(payload.text).await?;
    let sentence = Sentence {
        text: translation,
        start_time: payload.start_time,
        end_time: payload.end_time,
    };
    Ok(sentence)
}

fn setup_env(args: &Args) {
    env::set_var("DEEPL_BASE_URL", DEEPL_BASE_URL);

    env::set_var("ASSEMBLY_AI_BASE_URL", ASSEMBLY_AI_BASE_URL);
    env::set_var("ASSEMBLY_AI_API_KEY", "test");

    env::set_var("API_URL", "");
    env::set_var("ASSEMBLY_AI_WEBHOOK_ENDPOINT", "");
    env::set_var("ASSEMBLY_AI_WEBHOOK_TOKEN", "");

    let keys = match keys::Keys::new(&args.keys) {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    env::set_var("DEEPL_API_KEY", keys.deepl);
    env::set_var("ASSEMBLY_AI_API_KEY", keys.assembly_ai);
}
