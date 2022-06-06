use std::path::Path;
use std::sync::MutexGuard;
use std::time::Duration;

use path_providing::default_path_provider::PathProvider;
use path_providing::path_provider::PodcastEpisode;

use log::{error, warn};

use crate::player_error::{ErrorKind as PlayerErrorKind, PlayerError};

pub trait Mp3Player {
    fn get_path_provider(&self) -> MutexGuard<Box<dyn PathProvider>>;
    fn get_selected_episode(&self) -> &Option<PodcastEpisode>;
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>);
    fn pause(&mut self);
    fn play(&mut self);
    fn is_paused(&self) -> bool;

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError>;

    //fn get_selected_episode_duration(&self) -> Duration;

    fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), PlayerError> {
        if !self
            .get_path_provider()
            .compute_episode_path(episode)
            .exists()
        {
            warn!("Cannot select an episode which has not been downloaded first");
            return Err(PlayerError::new(None, PlayerErrorKind::FileNotFound));
        }
        self.set_selected_episode(Some(episode.clone()));
        Ok(())
    }

    fn play_selected_episode(&mut self) -> Result<(), PlayerError> {
        let path = self
            .get_path_provider()
            .compute_episode_path(self.get_selected_episode().as_ref().unwrap())
            .into_os_string()
            .into_string()
            .unwrap();

        if !Path::new(&path).exists() {
            error!("Could not find file {path}, playing failed");
            return Err(PlayerError::new(None, PlayerErrorKind::FileNotFound));
        }

        self.play_file(&path)
    }
}
