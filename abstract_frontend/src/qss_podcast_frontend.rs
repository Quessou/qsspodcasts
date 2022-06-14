pub use async_trait::async_trait;

#[async_trait]
pub trait QssPodcastFrontend {
    async fn run(&mut self) -> Result<(), ()>;
}
