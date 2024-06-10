use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use std::{path::PathBuf, sync::Weak};

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

use crate::player_error::PlayerError;
use crate::{duration_wrapper::DurationWrapper, traits::PlayerObserver};

use super::mp3_player::Mp3Player;

struct GStreamerPlayerState {
    // TODO: Make this async-friendly
    pub selected_episode: Arc<RwLock<PodcastEpisode>>,
    pub info: DiscovererInfo,
}

/*impl GStreamerPlayerState {
    pub async fn get_selected_episode(&self) -> RwLockReadGuard<'_, PodcastEpisode> {
        self.selected_episode.read().await
    }
}*/

pub struct GStreamerMp3Player {
    player_state: Option<Arc<RwLock<GStreamerPlayerState>>>,
    path_provider: Arc<dyn PathProvider + Send + Sync>,
    is_paused: bool,
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
                if play_state != PlayState::Stopped {
                    return;
                }
                let player_cloned_ptr = player_cloned_ptr.clone();
                let b = async move {
                    let player_cloned = player_cloned_ptr.clone();
                    let locked_player = player_cloned.lock().await;
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
                        let observer = o.clone().upgrade();
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
            is_paused: true,
            player,
            signal_catcher: Box::pin(signal_catcher),
            observers: vec![],
        }
    }

    fn reset_progression(&mut self) {
        self.player.seek(ClockTime::default());
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
    async fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
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
            return;
        }
        self.reset_progression();

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
    }

    fn pause(&mut self) {
        self.player.pause();
        self.is_paused = true;
    }

    fn play(&mut self) {
        self.player.play();
        self.is_paused = false;
    }

    async fn seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError> {
        match self.player.position() {
            Some(p) => {
                let p = p.seconds();
                let offset = duration.num_seconds();
                let episode_duration = self.get_selected_episode_duration().await;

                let episode_duration = episode_duration.unwrap().inner_ref().as_secs();
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

    fn is_paused(&self) -> bool {
        self.is_paused
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

        Some(
            (episode_progression * 100 / episode_duration)
                .try_into()
                .unwrap(),
        )
    }

    async fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), PlayerError> {
        if !self.compute_episode_path(episode).exists() {
            warn!("Cannot select an episode which has not been downloaded first");
            return Err(PlayerError::new(
                None,
                crate::player_error::ErrorKind::FileNotFound,
            ));
        }
        self.set_selected_episode(Some(episode.clone())).await;
        Ok(())
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
}

unsafe impl Send for GStreamerMp3Player {}
