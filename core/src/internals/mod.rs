pub mod cloud;
pub mod subtitler;
pub mod transcriber;
pub mod translator;
pub mod yt_downloader;
pub mod youtube_client;

pub trait ServiceProvider {
    fn id(&self) -> i32;
}
