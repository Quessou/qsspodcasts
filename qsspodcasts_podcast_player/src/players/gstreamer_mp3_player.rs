use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::{path::PathBuf, sync::Weak};

use chrono::TimeDelta;
use gstreamer_play::PlayState;
use gstreamer_play::{
    self,
    gst::{init, ClockTime},
    Play as GStreamerInnerPlayer, PlaySignalAdapter, PlayVideoRenderer,
};
use podcast_management::data_objects::hashable::Hashable;
use tokio::runtime::Handle;
use tokio::sync::{Mutex, RwLock};
use tokio::task::spawn;

use gstreamer_pbutils::{Discoverer, DiscovererInfo};

use log::{error, warn};
use path_providing::path_provider::PathProvider;
use path_providing::path_provider::PodcastEpisode;

use crate::enums::player_state::Mp3PlayerState;
use crate::player_error::{self, PlayerError};
use crate::{duration_wrapper::DurationWrapper, traits::PlayerObserver};

use super::mp3_player::Mp3Player;

struct GStreamerPlayerState {
    pub selected_episode: Arc<RwLock<PodcastEpisode>>,
    pub info: DiscovererInfo,
}

pub struct GStreamerMp3Player {
    player_state: Option<Arc<RwLock<GStreamerPlayerState>>>,
    path_provider: Arc<dyn PathProvider + Send + Sync>,
    play_state: Option<PlayState>,
    player: GStreamerInnerPlayer,
    signal_catcher: Pin<Box<PlaySignalAdapter>>,
    observers: Vec<Weak<Mutex<dyn PlayerObserver + Send + Sync>>>,
}

impl GStreamerMp3Player {
    pub async fn build(path_provider: Arc<dyn PathProvider + Send + Sync>) -> Arc<Mutex<Self>> {
        let player = Arc::new(Mutex::new(Self::new(path_provider)));

        let player_cloned_ptr = player.clone();
        let handle = Handle::current();

        player
            .lock()
            .await
            .signal_catcher
            .connect_state_changed(move |_, play_state| {
                let player_cloned = player_cloned_ptr.clone();
                let set_state = async move {
                    player_cloned.lock().await.play_state = Some(play_state);
                };
                handle.spawn(set_state);
                // TODO(mmiko) : Change this so that we can handle different states
                if play_state != PlayState::Stopped {
                    return;
                }
                let player_cloned = player_cloned_ptr.clone();
                let b = async move {
                    let player_cloned = player_cloned.clone();
                    let locked_player = player_cloned.lock().await;
                    let podcast_progression = locked_player
                        .get_selected_episode_progression()
                        .await
                        .expect(
                            "Tried to retrieve podcast progression while no podcast is selected",
                        );
                    let podcast_duration = locked_player
                        .get_selected_episode_duration()
                        .await
                        .expect("Tried to retrieve podcast duration while no podcast is selected");
                    if podcast_progression < podcast_duration {
                        return;
                    }
                    let hash = locked_player
                        .get_selected_episode()
                        .await
                        .unwrap()
                        .read()
                        .await
                        .hash();
                    let observers = &locked_player.observers;
                    observers.iter().for_each(move |o| {
                        let hash = hash.clone();
                        let observer = o.upgrade();
                        let notify_observer = async move {
                            let observer_unwrapped = observer.unwrap();
                            let mut observer_locked = observer_unwrapped.lock().await;
                            observer_locked.on_podcast_finished(&hash).await;
                        };
                        spawn(notify_observer);
                    });
                };

                handle.spawn(b);
            });

        player
    }

    pub fn new(path_provider: Arc<dyn PathProvider + Send + Sync>) -> Self {
        init().unwrap();
        let player = GStreamerInnerPlayer::new(None::<PlayVideoRenderer>);
        let signal_catcher = PlaySignalAdapter::new_sync_emit(&player);
        GStreamerMp3Player {
            player_state: None,
            path_provider,
            play_state: None,
            player,
            signal_catcher: Box::pin(signal_catcher),
            observers: vec![],
        }
    }

    async fn reset_state(&mut self) {
        self.player.seek(ClockTime::from_seconds(0));
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        self.player.stop();
    }
}

#[async_trait::async_trait]
impl Mp3Player for GStreamerMp3Player {
    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf {
        (*self.path_provider).compute_episode_path(episode)
    }

    async fn get_selected_episode(&self) -> Option<Arc<RwLock<PodcastEpisode>>> {
        if self.player_state.is_none() {
            None
        } else {
            let selected_episode = self
                .player_state
                .as_ref()
                .unwrap()
                .read()
                .await
                .selected_episode
                .clone();
            Some(selected_episode)
        }
    }
    async fn set_selected_episode(
        &mut self,
        episode: Option<PodcastEpisode>,
    ) -> Result<(), PlayerError> {
        if (episode.is_none() && self.player_state.is_none())
            || (self.player_state.is_some()
                && episode.as_ref().unwrap().hash()
                    == (*self.player_state.as_ref().unwrap())
                        .read()
                        .await
                        .selected_episode
                        .read()
                        .await
                        .hash())
        {
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::EpisodeAlreadySelected,
            ));
        }

        self.reset_state().await;

        if let Some(episode) = episode {
            let tmp_path = self.compute_episode_path(&episode);
            let tmp_path = tmp_path.into_os_string().into_string();
            let tmp_path = tmp_path.unwrap();
            let tmp_path = &format!("file://{}", &tmp_path);
            let path = Some(tmp_path);

            let discoverer = Discoverer::new(ClockTime::from_mseconds(1000)).unwrap();
            let info = discoverer.discover_uri(tmp_path).unwrap();
            self.player_state = Some(Arc::new(RwLock::new(GStreamerPlayerState {
                selected_episode: Arc::new(RwLock::new(episode)),
                info,
            })));

            self.player.set_uri(path.map(|x| &**x));
        } else {
            self.player_state = None;
            self.player.set_uri(None);
        }
        Ok(())
    }

    fn reset_progression(&mut self) {
        self.player.seek(ClockTime::from_seconds(0));
    }
    fn pause(&mut self) {
        self.player.pause();
    }

    fn play(&mut self) {
        self.player.play();
    }
    async fn absolute_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError> {
        let offset = ClockTime::from_seconds(duration.num_seconds() as u64);
        self.player.seek(offset);
        Ok(())
    }

    async fn relative_seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError> {
        match self.player.position() {
            Some(p) => {
                let p = p.seconds() as i64;
                let offset = duration.num_seconds();
                let episode_duration = self.get_selected_episode_duration().await;

                let episode_duration = episode_duration.unwrap().inner_ref().as_secs() as i64;
                let p: i64 = if offset + (p as i64) < 0 {
                    0
                } else if offset > 0 && (offset as i64) + p > episode_duration {
                    episode_duration
                } else if offset < 0 {
                    p - (-offset)
                } else {
                    p + (offset)
                };
                let r = self.absolute_seek(chrono::Duration::seconds(p)).await;
                if let Some(PlayState::Paused) = self.play_state {
                    if p == episode_duration {
                        self.play();
                    }
                }

                r
            }
            None => {
                log::error!("Trying to seek even though the player returned no position");
                Ok(())
            }
        }
    }

    fn is_paused(&self) -> bool {
        let state = self.get_state();
        state == Mp3PlayerState::Paused || state == Mp3PlayerState::Stopped
    }

    fn play_file(&mut self, path: &str) -> Result<(), PlayerError> {
        self.player.set_uri(Some(&format!("file://{}", path)));
        self.play();
        Ok(())
    }

    fn register_observer(&mut self, observer: Weak<Mutex<dyn PlayerObserver + Send + Sync>>) {
        self.observers.push(observer);
    }

    async fn get_selected_episode_duration(&self) -> Option<DurationWrapper> {
        self.get_selected_episode().await?;

        let duration = self
            .player_state
            .as_ref()
            .unwrap()
            .read()
            .await
            .info
            .duration()
            .unwrap();
        let duration = Duration::new(duration.seconds(), 0);
        Some(DurationWrapper::new(duration))
    }

    async fn get_selected_episode_progression(&self) -> Option<DurationWrapper> {
        self.get_selected_episode().await?;
        let progression = self.player.position().unwrap_or_default();

        let progression = Duration::new(progression.seconds(), 0);
        Some(DurationWrapper::new(progression))
    }

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

        assert_ne!(episode_duration, 0);
        Some(
            (episode_progression * 100 / episode_duration)
                .try_into()
                .unwrap(),
        )
    }

    async fn play_selected_episode(&mut self) -> Result<(), PlayerError> {
        let selected_episode = self.get_selected_episode().await;
        let selected_episode_lock = selected_episode.as_ref().unwrap().read().await;
        let selected_episode_ref = &selected_episode_lock.to_owned();
        let path = self
            .compute_episode_path(selected_episode_ref)
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

    fn get_state(&self) -> Mp3PlayerState {
        if self.play_state.is_none() {
            Mp3PlayerState::Stopped
        } else {
            self.play_state.unwrap().into()
        }
    }
}

unsafe impl Send for GStreamerMp3Player {}
