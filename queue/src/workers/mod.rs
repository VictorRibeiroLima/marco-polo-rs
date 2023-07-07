use async_trait::async_trait;
pub mod heavy;
pub mod light;

#[async_trait]
pub trait Worker {
    async fn handle_queue(&self);
}
