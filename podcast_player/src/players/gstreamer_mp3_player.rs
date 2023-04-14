use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gstreamer_play::{
    self,
    gst::{init, ClockTime},
    Play as GStreamerInnerPlayer, PlayVideoRenderer,
};

use gstreamer_pbutils::{Discoverer, DiscovererInfo};

use log::{error, warn};
use path_providing::path_provider::PathProvider;
use path_providing::path_provider::PodcastEpisode;

use crate::duration_wrapper::DurationWrapper;
use crate::player_error::PlayerError;

use super::mp3_player::Mp3Player;

struct GStreamerPlayerState {
    pub selected_episode: PodcastEpisode,
    pub info: DiscovererInfo,
}

pub struct GStreamerMp3Player {
    player_state: Option<GStreamerPlayerState>,
    path_provider: Arc<Mutex<Box<dyn PathProvider>>>,
    is_paused: bool,
    player: GStreamerInnerPlayer,
}

impl GStreamerMp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> Self {
        init().unwrap();
        let player = GStreamerInnerPlayer::new(None::<PlayVideoRenderer>);
        GStreamerMp3Player {
            player_state: None,
            path_provider: Arc::new(Mutex::new(path_provider)),
            is_paused: true,
            player,
        }
    }

    fn reset_progression(&mut self) {
        self.player.seek(ClockTime::default());
    }
}

impl Mp3Player for GStreamerMp3Player {
    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf {
        self.path_provider
            .lock()
            .unwrap()
            .compute_episode_path(episode)
    }
    fn get_selected_episode(&self) -> Option<&PodcastEpisode> {
        if self.player_state.is_none() {
            None
        } else {
            Some(&self.player_state.as_ref().unwrap().selected_episode)
        }
    }
    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
        if (episode.is_none() && self.player_state.is_none())
            || (self.player_state.is_some()
                && episode.as_ref() == Some(&self.player_state.as_ref().unwrap().selected_episode))
        {
            return;
        }
        self.reset_progression();

        if episode.is_none() {
            self.player_state = None;
            self.player.set_uri(None);
        } else {
            let tmp_path = self.compute_episode_path(episode.as_ref().unwrap());
            let tmp_path = tmp_path.into_os_string().into_string();
            let tmp_path = tmp_path.unwrap();
            let tmp_path = &format!("file://{}", &tmp_path);
            let path = Some(tmp_path);

            let discoverer = Discoverer::new(ClockTime::from_mseconds(1000)).unwrap();
            let info = discoverer.discover_uri(tmp_path).unwrap();
            self.player_state = Some(GStreamerPlayerState {
                selected_episode: episode.unwrap(),
                info,
            });

            self.player.set_uri(path.map(|x| &**x));
        }
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

    fn seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError> {
        match self.player.position() {
            Some(p) => {
                let p = p.seconds();
                let offset = duration.num_seconds();
                let episode_duration = self.player.duration().unwrap().seconds();
                let p: u64 = if offset + (p as i64) < 0 {
                    0
                } else if offset > 0 && (offset as u64) + p > episode_duration {
                    episode_duration
                } else if offset < 0 {
                    p - ((-offset) as u64)
                } else {
                    p + (offset as u64)
                };
                self.player.seek(ClockTime::from_seconds(p));
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn get_selected_episode_duration(&self) -> Option<DurationWrapper> {
        if self.get_selected_episode().is_none() {
            return None;
        }

        let duration = self.player_state.as_ref().unwrap().info.duration().unwrap();
        let duration = Duration::new(duration.seconds(), 0);
        Some(DurationWrapper::new(duration))
    }

    fn get_selected_episode_progression(&self) -> Option<DurationWrapper> {
        if self.get_selected_episode().is_none() {
            return None;
        }

        let progression = self.player.position().unwrap_or_default();

        let progression = Duration::new(progression.seconds(), 0);
        Some(DurationWrapper::new(progression))
    }

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
            return Err(PlayerError::new(
                None,
                crate::player_error::ErrorKind::FileNotFound,
            ));
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

        if !std::path::Path::new(&path).exists() {
            error!("Could not find file {path}, playing failed");
            return Err(PlayerError::new(
                None,
                crate::player_error::ErrorKind::FileNotFound,
            ));
        }

        self.play_file(&path)
    }
}

unsafe impl Send for GStreamerMp3Player {}
