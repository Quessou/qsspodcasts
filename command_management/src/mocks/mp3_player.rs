use mockall::mock;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_error::PlayerError;
use podcast_player::players::mp3_player::Mp3Player as TraitMp3Player;
use std::path::PathBuf;

mock! {
    pub Mp3Player {}     // Name of the mock struct, less the "Mock" prefix
    impl TraitMp3Player for Mp3Player {   // specification of the trait to mock
        fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf;
        fn get_selected_episode(&self) -> &Option<PodcastEpisode>;
        fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>);
        fn pause(&mut self);
        fn play(&mut self);
        fn is_paused(&self) -> bool;
        fn play_file(&mut self, path: &str) -> Result<(), PlayerError>;
        fn get_selected_episode_duration(&self) -> Option<DurationWrapper>;
        fn get_selected_episode_progression(&self) -> Option<DurationWrapper>;
    }
}
unsafe impl Send for MockMp3Player {}
