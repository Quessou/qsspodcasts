use std::path::PathBuf;
use std::sync::{Arc, Weak};

use tokio::sync::RwLock;

use podcast_player::enums::player_state::Mp3PlayerState;

use mockall::mock;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_error::PlayerError;
use podcast_player::players::mp3_player::Mp3Player as TraitMp3Player;
use podcast_player::traits::PlayerObserver;

use async_trait::async_trait;

mock! {
    pub Mp3Player {}     // Name of the mock struct, less the "Mock" prefix
    #[async_trait]
    impl TraitMp3Player for Mp3Player {   // specification of the trait to mock
        fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf;
        async fn get_selected_episode(&self) -> Option<Arc<RwLock<PodcastEpisode>>>;
        async fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) -> Result<(), PlayerError>;
        fn pause(&mut self);
        fn play(&mut self);
        async fn relative_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError>;
        async fn absolute_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError>;
        fn reset_progression(&mut self);
        fn is_paused(&self) -> bool;
        fn play_file(&mut self, path: &str) -> Result<(), PlayerError>;
        async fn get_selected_episode_duration(&self) -> Option<DurationWrapper>;
        async fn get_selected_episode_progression(&self) -> Option<DurationWrapper>;
        fn register_observer(&mut self, observer: Weak<tokio::sync::Mutex<dyn PlayerObserver + Send + Sync>>);
        fn get_state(&self) -> Mp3PlayerState;
        fn set_volume(&mut self, volume: u32) -> Result<(), PlayerError>;
        fn get_volume(&self) -> u32;
        fn add_volume_offset(&mut self, volume: i32) -> Result<(), PlayerError>;
    }
}
unsafe impl Send for MockMp3Player {}
unsafe impl Sync for MockMp3Player {}
