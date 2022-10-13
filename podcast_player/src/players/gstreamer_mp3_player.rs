use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gstreamer::ClockTime;
use gstreamer_player::{self, Player as GStreamerInnerPlayer};

use path_providing::path_provider::PathProvider;
use path_providing::path_provider::PodcastEpisode;

use crate::duration_wrapper::DurationWrapper;
use crate::player_error::PlayerError;

use super::mp3_player::Mp3Player;

pub struct GStreamerMp3Player {
    selected_episode: Option<PodcastEpisode>,
    path_provider: Arc<Mutex<Box<dyn PathProvider>>>,
    is_paused: bool,
    player: GStreamerInnerPlayer, // TODO : Add stuff
}

impl GStreamerMp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> GStreamerMp3Player {
        GStreamerMp3Player {
            selected_episode: None,
            path_provider: Arc::new(Mutex::new(path_provider)),
            is_paused: true,
            player: GStreamerInnerPlayer::new(
                None,
                Some(gstreamer_player::PlayerGMainContextSignalDispatcher::new(None).as_ref()),
            ),
        }
    }
}

impl Mp3Player for GStreamerMp3Player {
    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf {
        self.path_provider
            .lock()
            .unwrap()
            .compute_episode_path(episode)
    }
    fn get_selected_episode(&self) -> &Option<PodcastEpisode> {
        &self.selected_episode
    }
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
        self.selected_episode = episode;
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError> {
        self.player.set_uri(Some(&format!("file://{}", path)));
        self.play();
        Ok(())
    }

    fn pause(&mut self) {
        self.player.pause();
        self.is_paused = true;
    }

    fn play(&mut self) {
        self.player.play();
        self.is_paused = false;
    }

    fn get_selected_episode_duration(&self) -> Option<DurationWrapper> {
        if self.get_selected_episode().is_none() {
            return None;
        }

        let duration = self.player.duration().unwrap_or(ClockTime::default());

        let duration = Duration::new(duration.seconds(), 0);
        Some(DurationWrapper::new(duration))
    }

    fn get_selected_episode_progression(&self) -> Option<DurationWrapper> {
        if self.get_selected_episode().is_none() {
            return None;
        }

        let progression = self.player.position().unwrap_or(ClockTime::default());

        let progression = Duration::new(progression.seconds(), 0);
        Some(DurationWrapper::new(progression))
    }
}

unsafe impl Send for GStreamerMp3Player {}
