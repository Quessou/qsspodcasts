use log::{error, warn};
use std::sync::{Arc, Mutex, MutexGuard};

use path_providing::path_provider::PathProvider;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

use crate::mp3_player::Mp3Player;
use crate::player_error::PlayerError;

pub struct SymphoniaMp3Player {
    selected_episode: Option<PodcastEpisode>,
    path_provider: Arc<Mutex<Box<dyn PathProvider>>>,
}

impl SymphoniaMp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> SymphoniaMp3Player {
        SymphoniaMp3Player {
            selected_episode: None,
            path_provider: Arc::new(Mutex::new(path_provider)),
        }
    }
}

impl Mp3Player for SymphoniaMp3Player {
    fn get_path_provider(&self) -> MutexGuard<Box<dyn PathProvider>> {
        self.path_provider.lock().unwrap()
    }
    fn get_selected_episode(&self) -> &Option<PodcastEpisode> {
        &self.selected_episode
    }
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
        self.selected_episode = episode;
    }

    fn is_paused(&self) -> bool {
        false
    }

    fn play_file(&mut self, _path: &str) -> Result<(), PlayerError> {
        // TODO: Implement this method
        Ok(())
    }

    fn pause(&mut self) {}
    fn play(&mut self) {}
}

unsafe impl Send for SymphoniaMp3Player {}
