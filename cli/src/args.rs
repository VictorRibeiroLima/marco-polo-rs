use clap::Parser;

/// The Marco polo rs CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the video file
    #[arg(short, long, default_value = "input.mp4")]
    pub input: String,

    /// Path to the output video file, use the same extension as the input file
    #[arg(short, long, default_value = "output.mp4")]
    pub output: String,

    /// Path to the api keys file
    #[arg(short, long, default_value = "./api_keys.json")]
    pub keys: String,
}
