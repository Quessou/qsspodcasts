pub trait PlayerObserver {
    fn on_podcast_finished(&mut self, hash: &str);
}
