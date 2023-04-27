pub mod cloud;
pub mod transcriber;

pub trait ServiceProvider {
    fn id() -> u32;
}
