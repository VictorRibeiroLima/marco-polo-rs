pub mod cloud;
pub mod transcriber;

pub trait ServiceProvider {
    fn id() -> i32;
}
