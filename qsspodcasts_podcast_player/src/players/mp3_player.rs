use std::path::{Path, PathBuf};
use std::sync::{Arc, Weak};
use std::time::Duration;

use tokio::sync::{Mutex, RwLock};

use path_providing::path_provider::PodcastEpisode;

use chrono;
use log::{error, warn};

use crate::enums::player_state::Mp3PlayerState;
use crate::traits::PlayerObserver;
use crate::{
    duration_wrapper::DurationWrapper,
    player_error::{ErrorKind as PlayerErrorKind, PlayerError},
};

#[async_trait::async_trait]
pub trait Mp3Player {
    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf;
    async fn get_selected_episode(&self) -> Option<Arc<RwLock<PodcastEpisode>>>;
    async fn set_selected_episode(
        &mut self,
        episode: Option<PodcastEpisode>,
    ) -> Result<(), PlayerError>;
    fn pause(&mut self);
    fn play(&mut self);
    fn reset_progression(&mut self);
    async fn relative_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError>;
    async fn absolute_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError>;
    fn is_paused(&self) -> bool;

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError>;
    fn register_observer(&mut self, observer: Weak<Mutex<dyn PlayerObserver + Send + Sync>>);
    fn get_state(&self) -> Mp3PlayerState;

    async fn get_selected_episode_duration(&self) -> Option<DurationWrapper>;
    async fn get_selected_episode_progression(&self) -> Option<DurationWrapper>;
    async fn get_selected_episode_progression_percentage(&self) -> Option<u8> {
        let episode_duration: Duration = match self.get_selected_episode_duration().await {
            Some(d) => d.into(),
            None => return None,
        };
        let episode_duration = episode_duration.as_secs();

        if episode_duration == 0 {
            return Some(0);
        }

        let episode_progression: Duration = self
            .get_selected_episode_progression()
            .await
            .unwrap_or_default()
            .into();
        let episode_progression = episode_progression.as_secs();

        Some(
            (episode_progression * 100 / episode_duration)
                .try_into()
                .unwrap(),
        )
    }

    async fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), PlayerError> {
        if !self.compute_episode_path(episode).exists() {
            warn!("Cannot select an episode which has not been downloaded first");
            return Err(PlayerError::new(None, PlayerErrorKind::FileNotFound));
        }
        self.reset_progression();
        self.set_selected_episode(Some(episode.clone())).await
    }

    async fn play_selected_episode(&mut self) -> Result<(), PlayerError> {
        let selected_episode = self.get_selected_episode().await;
        let selected_episode_lock_guard = selected_episode.as_ref().unwrap().read().await;
        let selected_episode_ref = &selected_episode_lock_guard.to_owned();

        let path = self
            .compute_episode_path(selected_episode_ref)
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
