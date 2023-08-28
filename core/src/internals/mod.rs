pub mod cloud;
pub mod subtitler;
pub mod transcriber;
pub mod translator;
pub mod video_platform;
pub mod yt_downloader;

pub trait ServiceProvider {
    fn id(&self) -> i32;
}
