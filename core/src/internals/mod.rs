pub mod cloud;
pub mod subtitler;
pub mod transcriber;
pub mod translator;
pub mod yt_downloader;

pub trait ServiceProvider {
    fn id(&self) -> i32;
}
