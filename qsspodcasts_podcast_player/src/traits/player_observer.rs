use async_trait::async_trait;

#[async_trait]
pub trait PlayerObserver {
    async fn on_podcast_finished(&mut self, hash: &str);
    //async fn on_volume_changed(&mut self, new_volume: u32);
}
