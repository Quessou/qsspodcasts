use async_trait::async_trait;

#[async_trait]
pub trait PlayerObserver {
    async fn on_podcast_finished(&mut self, hash: &str);
}
