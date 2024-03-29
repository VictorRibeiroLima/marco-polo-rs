use clap::Parser;

/// The Marco polo rs CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the video file
    #[arg(short, long, default_value = "./input.mp4")]
    pub input: String,

    /// Path to the output video file or srt file if srt_only is on.
    /// Use the same extension as the input file
    #[arg(short, long, default_value = "./output.mp4")]
    pub output: String,

    /// Path to the api keys file
    #[arg(short, long, default_value = "./api_keys.json")]
    pub keys: String,

    /// Whether or not to render the video with subtitles or just the srt file
    #[arg(long, default_value = "false")]
    pub srt_only: bool,

    /// Define which translation service to use (google or deepl)
    #[arg(short, long, default_value = "google")]
    pub translation_service: String,
}
