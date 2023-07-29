use std::{fs::File, io::Write};

use marco_polo_rs_core::{
    internals::{
        transcriber::traits::Sentence,
        translator::{
            deepl::DeeplClient, google_v2::GoogleTranslateV2Client, traits::TranslatorClient,
        },
    },
    SyncError,
};

use crate::args::Args;

pub async fn get_srt_string(sentences: Vec<Sentence>, args: &Args) -> Result<String, ()> {
    let srt_file_string: String;

    if args.translation_service == "deepl" {
        let client = DeeplClient::new();
        srt_file_string = match get_srt_file_string(sentences, client).await {
            Ok(srt_file_string) => srt_file_string,
            Err(e) => {
                eprintln!("{}", e);
                return Err(());
            }
        };
    } else if args.translation_service == "google" {
        let client = GoogleTranslateV2Client::new();
        srt_file_string = match get_srt_file_string(sentences, client).await {
            Ok(srt_file_string) => srt_file_string,
            Err(e) => {
                eprintln!("{}", e);
                return Err(());
            }
        };
    } else {
        eprintln!(
            "Translation service '{}' not supported",
            args.translation_service
        );
        return Err(());
    }

    return Ok(srt_file_string);
}

pub fn write_srt_file(srt_path: &str, srt: String) -> Result<(), ()> {
    match File::create(&srt_path) {
        Ok(mut file) => match file.write_all(srt.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Unable to write to {}: {}", srt_path, e);
                Err(())
            }
        },
        Err(_) => {
            eprintln!(
                "Unable to create {} pls put it in the same directory as the executable",
                srt
            );
            Err(())
        }
    }
}

async fn get_srt_file_string(
    mut sentences: Vec<Sentence>,
    translator_client: impl TranslatorClient,
) -> Result<String, SyncError> {
    let string_sentences = sentences
        .iter()
        .map(|s| s.text.as_str())
        .collect::<Vec<&str>>();

    let translated_sentences = translator_client
        .translate_sentences(string_sentences)
        .await?;

    for (i, translation) in translated_sentences.into_iter().enumerate() {
        sentences[i].text = translation.to_string();
    }

    println!("Creating new srt file...");
    let new_srt_buffer = marco_polo_rs_core::util::srt::create_based_on_sentences(sentences);
    Ok(new_srt_buffer)
}
