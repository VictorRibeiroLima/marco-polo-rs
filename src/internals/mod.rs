pub mod cloud;
pub mod subtitler;
pub mod transcriber;
pub mod translator;

pub trait ServiceProvider {
    fn id() -> i32;
}
