use std::borrow::BorrowMut;
use std::io::{self, Error as IoError};
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use fs_utils::{progression_read_utils, write_utils};
use log::{error, info};
use podcast_management::data_objects::hashable::Hashable;
use podcast_player::enums::player_state::Mp3PlayerState;
use podcast_player::player_error;
use podcast_player::traits::PlayerObserver;
use tokio::sync::Mutex as TokioMutex;

use rss_management::{
    local_storage::{
        application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider,
    },
    url_storage::file_url_storer::FileUrlStorer,
};

use crate::event_type::EventType;
use crate::notification::Notification;
use data_transport::DataSender;
use path_providing::default_path_provider::PathProvider;
use podcast_download::podcast_downloader::PodcastDownloader;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;
use podcast_management::{
    builders::podcast_builder::PodcastBuilder, data_objects::podcast::Podcast,
    podcast_library::PodcastLibrary,
};
use podcast_player::{player_error::PlayerError, players::mp3_player::Mp3Player};

// TODO : Add concept of InitializedBusinessCore which is returned by BusinessCore::initialize, and consumes self
pub struct BusinessCore {
    application_dir_initializer: ApplicationDirInitializer,
    rss_provider: RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
    pub podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    path_provider: Arc<dyn PathProvider + Send + Sync>,
    notifications_sender: Option<DataSender<Notification>>,
}

impl BusinessCore {
    pub fn new(
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
        path_provider: Arc<dyn PathProvider + Send + Sync>,
        notifications_sender: Option<DataSender<Notification>>,
    ) -> BusinessCore {
        let podcast_library = Arc::new(TokioMutex::new(PodcastLibrary::new()));
        let core = BusinessCore {
            rss_provider: RssProvider::new(FileUrlStorer::new(PathBuf::from(
                path_provider.rss_feed_list_file_path().to_str().unwrap(),
            ))),
            podcast_builder: PodcastBuilder::new(),
            podcast_downloader: PodcastDownloader::new(path_provider.clone()),
            player: mp3_player,
            podcast_library,
            application_dir_initializer: ApplicationDirInitializer {
                path_provider: path_provider.clone(),
            },
            path_provider,
            notifications_sender,
        };
        // TODO: plug behavior on state change of player ? Probably not here
        core
    }

    pub async fn new_in_arc(
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
        path_provider: Arc<dyn PathProvider + Send + Sync>,
        notifications_sender: Option<DataSender<Notification>>,
    ) -> Arc<TokioMutex<BusinessCore>> {
        let core = Arc::new(TokioMutex::new(Self::new(
            mp3_player,
            path_provider,
            notifications_sender,
        )));
        let mut core_ref = core.as_ref();
        let tmp_core = core_ref.borrow_mut();
        let tmp_core_locked = tmp_core.lock().await;
        let mut player = tmp_core_locked.player.lock().await;
        let casted_core: Arc<TokioMutex<dyn PlayerObserver + Send + Sync>> = core.clone();
        player.register_observer(Arc::downgrade(&casted_core));
        drop(casted_core);
        drop(player);
        drop(tmp_core_locked);
        core
    }

    pub fn initialize(&self) {
        let app_dir_path = self.path_provider.app_dir_path();
        let app_dir_path = app_dir_path.to_str().unwrap();
        if !self
            .application_dir_initializer
            .is_app_dir_created(PathBuf::from(app_dir_path))
        {
            self.application_dir_initializer
                .initialize_application_dir(app_dir_path)
                .expect("Application dir initialization failed");
        }
    }

    pub async fn add_url(&mut self, url: &str) -> Result<(), IoError> {
        if let Err(e) = self.rss_provider.add_url(url) {
            self.send_notification(Notification::Message(
                "Writing of URL failed (already added ?)".to_string(),
            ))
            .await;
            return Err(e);
        }
        info!("Url added successfully");
        self.send_notification(Notification::Message(format!(
            "Url {} added successfully",
            url
        )))
        .await;
        Ok(())
    }

    pub async fn delete_rss(&mut self, hash: &str) -> Result<(), IoError> {
        let mut library = self.podcast_library.lock().await;
        let podcast = library.search_podcast(hash);
        library.delete_podcast(hash)?;
        drop(library);

        if podcast.is_none() {
            self.send_notification(Notification::Message(
                "Could not delete feed (hash does not exist ?)".to_string(),
            ))
            .await;
            return Err(IoError::new(
                io::ErrorKind::NotFound,
                "Could not find podcast matching hash",
            ));
        }
        let url = podcast.unwrap().link;
        if let Err(e) = self.rss_provider.delete_url(&url) {
            self.send_notification(Notification::Message("Deletion of URL failed".to_string()))
                .await;
            return Err(e);
        };
        self.send_notification(Notification::Message(
            "RSS feed deletion successful".to_string(),
        ))
        .await;
        Ok(())
    }

    pub async fn load_feed(&mut self, url: &str) -> Result<(), ()> {
        let feeds = self.rss_provider.get_all_feeds().await;
        let channel = feeds.0.iter().find(|c| c.0 == url);
        if channel.is_none() {
            error!("Could not find channel matching URL {}", url);
            return Err(());
        }
        let podcast = self.podcast_builder.build(&channel.unwrap().1);
        self.podcast_library.lock().await.push(podcast);
        Ok(())
    }

    pub async fn build_podcasts(&mut self) {
        self.send_notification(Notification::Message("Building library...".to_string()))
            .await;

        let channels = self.rss_provider.get_all_feeds().await;
        let mut podcasts: Vec<Podcast> = vec![];
        for channel in &channels.0 {
            podcasts.push(self.podcast_builder.build(&channel.1))
        }
        self.podcast_library.lock().await.push(podcasts);
        if !channels.1.is_empty() {
            let failed_feeds = channels.1.join(", ");
            self.send_notification(Notification::Message(format!(
                "Failed to download feeds for following urls : {}",
                failed_feeds
            )))
            .await;
        }
        self.send_notification(Notification::Message("Building library done".to_string()))
            .await;
    }

    pub async fn download_episode(&mut self, episode: &PodcastEpisode) -> Result<(), ()> {
        self.send_notification(Notification::Message(format!(
            "Downloading \"{}\"",
            episode.title
        )))
        .await;
        if (self.podcast_downloader.download_episode(episode).await).is_err() {
            self.send_notification(Notification::Message("Downloading failed".to_string()))
                .await;
            return Err(());
        }
        self.send_notification(Notification::Message("Downloading successful".to_string()))
            .await;

        Ok(())
    }

    async fn send_notification(&mut self, notification: Notification) {
        if self.notifications_sender.is_none() {
            return;
        }
        self.notifications_sender
            .as_mut()
            .unwrap()
            .send(notification)
            .await
            .expect("Writing notification in channel failed");
    }

    pub async fn seek(&mut self, duration: chrono::Duration) -> Result<(), PlayerError> {
        self.player.lock().await.relative_seek(duration).await
    }

    pub async fn play(&mut self) -> Result<(), PlayerError> {
        if self
            .player
            .lock()
            .await
            .get_selected_episode()
            .await
            .is_none()
        {
            self.send_notification(Notification::Message("No episode selected".to_owned()))
                .await;
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::NoEpisodeSelected,
            ));
        }
        let player_state = self.player.lock().await.get_state();
        if player_state == Mp3PlayerState::Stopped || player_state == Mp3PlayerState::Paused {
            self.player.lock().await.play();
            self.send_notification(Notification::Message("Player launched".to_owned()))
                .await;
        } else {
            self.send_notification(Notification::Message("Player already running".to_owned()))
                .await;
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::AlreadyPlaying,
            ));
        }
        Ok(())
    }

    async fn save_current_podcast_progression(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self
            .player
            .lock()
            .await
            .get_selected_episode()
            .await
            .is_none()
        {
            return Err(Box::new(PlayerError::new(
                None,
                player_error::ErrorKind::NoEpisodeSelected,
            )));
        }
        let hash = self
            .player
            .lock()
            .await
            .get_selected_episode()
            .await
            .unwrap()
            .read()
            .await
            .hash();
        let mut progression_file_path = self.path_provider.podcast_progresses_dir_path();
        progression_file_path.push(hash);
        let current_progression = self
            .player
            .lock()
            .await
            .get_selected_episode_progression().await
            .expect(
            "Tried to get progression of podcast while saving but no progression could be found",
        );
        fs_utils::progression_write_utils::write_progression_in_file(
            current_progression.inner_ref(),
            progression_file_path,
        )
        .await
        .expect("Writing of progression in file failed");
        Ok(())
    }

    pub async fn pause(&mut self) -> Result<(), PlayerError> {
        if self
            .player
            .lock()
            .await
            .get_selected_episode()
            .await
            .is_none()
        {
            self.send_notification(Notification::Message("No episode selected".to_owned()))
                .await;
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::NoEpisodeSelected,
            ));
        }
        // Actually pause the player
        if !self.player.lock().await.is_paused() {
            self.player.lock().await.pause();
            self.save_current_podcast_progression().await.unwrap();
            self.send_notification(Notification::Message("Player paused".to_string()))
                .await;
        } else {
            self.send_notification(Notification::Message("Player already paused".to_string()))
                .await;
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::AlreadyPaused,
            ));
        }

        Ok(())
    }

    async fn create_mark_as_finished_marker_file(&mut self, hash: &str) -> Result<(), ()> {
        let cloned_hash = hash.to_owned();
        let finished_podcast_file_path =
            self.path_provider.compute_finished_podcast_file_path(hash);
        let _ =
            write_utils::open_or_create_file(finished_podcast_file_path.to_str().unwrap()).unwrap();
        self.send_notification(Notification::Message("Episode finished".to_owned()))
            .await;
        self.send_notification(Notification::Event(EventType::PodcastFinished(cloned_hash)))
            .await;
        Ok(())
    }

    async fn delete_progression_file(&mut self, hash: &str) -> Result<(), ()> {
        let progression_file_path = self.path_provider.podcast_progress_file_path(hash);
        let r = tokio::fs::remove_file(progression_file_path).await;
        match r {
            Ok(_) => {}
            Err(e) => {
                let error_message = match e.kind() {
                    io::ErrorKind::NotFound => "Not found",
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    _ => "Unhandled error",
                };
                log::error!(
                    "Deletion of progression file failed. Error : {}",
                    error_message
                );
            }
        };
        Ok(())
    }

    #[allow(unused_assignments)]
    pub async fn mark_current_podcast_as_finished(&mut self) -> Result<(), PlayerError> {
        let mut hash: Option<String> = None;
        {
            let player = self.player.lock().await;
            let episode = player.get_selected_episode().await;
            hash = if let Some(e) = episode {
                Some(e.read().await.hash().clone())
            } else {
                None
            };
        }

        if hash.is_none() {
            self.send_notification(Notification::Message(
                "No episode currently selected".to_string(),
            ))
            .await;
            return Err(PlayerError::new(
                None,
                player_error::ErrorKind::NoEpisodeSelected,
            ));
        }
        match self
            .create_mark_as_finished_marker_file(hash.as_ref().unwrap())
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(PlayerError::new(None, player_error::ErrorKind::Other)),
        }
    }

    pub async fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), PlayerError> {
        let hash = episode.hash();
        let path = self.path_provider.podcast_progress_file_path(&hash);
        let duration = progression_read_utils::read_progression_in_file(path).await;

        if self.save_current_podcast_progression().await.is_err() {
            log::info!("Did not save current podcast progression due to no episode being selected (probably)");
        }
        self.player.lock().await.reset_progression();

        let r = self.player.lock().await.select_episode(episode).await;
        match r {
            Ok(_) => {
                self.send_notification(Notification::Message(
                    "Episode selection successful".to_string(),
                ))
                .await;
                self.send_notification(Notification::Event(EventType::PodcastLaunched(
                    episode.title.clone(),
                )))
                .await;
                if duration.is_some() {
                    assert!(
                        self.player
                            .lock()
                            .await
                            .get_selected_episode()
                            .await
                            .is_some(),
                        "The episode is actually not selected"
                    );
                    let duration: chrono::Duration =
                        chrono::Duration::seconds(duration.unwrap().as_secs() as i64);
                    self.player
                        .lock()
                        .await
                        .absolute_seek(duration)
                        .await
                        .expect("Seeking resuming position of podcast failed");
                }
            }
            Err(ref e) => {
                let error_kind = e.kind();
                let player_state = self.player.lock().await.get_state();
                if error_kind == player_error::ErrorKind::EpisodeAlreadySelected {
                    match player_state {
                        // If episode is already selected and finished, we want to reset the player
                        Mp3PlayerState::Stopped => {
                            self.send_notification(Notification::Message(
                                "Resetting podcast".to_owned(),
                            ))
                            .await;
                            self.player.lock().await.reset_progression();
                        }
                        _ => {
                            self.send_notification(Notification::Message(format!(
                                "Selecting already selected episode while in state {}",
                                player_state
                            )))
                            .await
                        }
                    }
                } else {
                    self.send_notification(Notification::Message(
                        "Episode selection failed".to_string(),
                    ))
                    .await
                }
            }
        };
        r
    }
    pub async fn clean(&mut self) {
        if self
            .player
            .lock()
            .await
            .get_selected_episode()
            .await
            .is_some()
        {
            self.send_notification(Notification::Message(
                "Writing progression of current podcast".to_string(),
            ))
            .await;
            self.save_current_podcast_progression()
                .await
                .expect("Cleaning failed");
        }
    }
    pub async fn set_volume(&mut self, new_volume: u32) -> Result<(), PlayerError> {
        self.player.lock().await.set_volume(new_volume)
    }
    pub async fn add_volume_offset(&mut self, volume_offset: i32) -> Result<(), PlayerError> {
        self.player.lock().await.add_volume_offset(volume_offset)
    }
}

#[async_trait]
impl PlayerObserver for BusinessCore {
    async fn on_podcast_finished(&mut self, hash: &str) {
        self.create_mark_as_finished_marker_file(hash)
            .await
            .unwrap();
        self.delete_progression_file(hash).await.unwrap();
    }
}

unsafe impl Send for BusinessCore {}
unsafe impl Sync for BusinessCore {}
