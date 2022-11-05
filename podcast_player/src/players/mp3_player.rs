use std::path::{Path, PathBuf};
use std::time::Duration;

use path_providing::path_provider::PodcastEpisode;

use chrono;
use log::{error, warn};

use crate::{
    duration_wrapper::DurationWrapper,
    player_error::{ErrorKind as PlayerErrorKind, PlayerError},
};
pub trait Mp3Player {
    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf;
    fn get_selected_episode(&self) -> &Option<PodcastEpisode>;
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>);
    fn pause(&mut self);
    fn play(&mut self);
    fn seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError>;
    fn is_paused(&self) -> bool;

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError>;

    fn get_selected_episode_duration(&self) -> Option<DurationWrapper>;
    fn get_selected_episode_progression(&self) -> Option<DurationWrapper>;
    fn get_selected_episode_progression_percentage(&self) -> Option<u8> {
        let episode_duration: Duration = match self.get_selected_episode_duration() {
            Some(d) => d.into(),
            None => return None,
        };
        let episode_duration = episode_duration.as_secs();

        if episode_duration == 0 {
            return Some(0);
        }

        let episode_progression: Duration = self
            .get_selected_episode_progression()
            .unwrap_or_default()
            .into();
        let episode_progression = episode_progression.as_secs();

        Some(
            (episode_progression * 100 / episode_duration)
                .try_into()
                .unwrap(),
        )
    }

    fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), PlayerError> {
        if !self.compute_episode_path(episode).exists() {
            warn!("Cannot select an episode which has not been downloaded first");
            return Err(PlayerError::new(None, PlayerErrorKind::FileNotFound));
        }
        self.set_selected_episode(Some(episode.clone()));
        Ok(())
    }

    fn play_selected_episode(&mut self) -> Result<(), PlayerError> {
        let path = self
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
