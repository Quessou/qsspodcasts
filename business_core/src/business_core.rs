use std::io::Error as IoError;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use log::{error, info};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex as TokioMutex;

use rss_management::{
    local_storage::{
        application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider,
    },
    url_storage::file_url_storer::FileUrlStorer,
};

use crate::notification::Notification;
use path_providing::default_path_provider::{DefaultPathProvider, PathProvider};
use podcast_download::podcast_downloader::PodcastDownloader;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;
use podcast_management::{
    builders::podcast_builder::PodcastBuilder, data_objects::podcast::Podcast,
    podcast_library::PodcastLibrary,
};
use podcast_player::players::{gstreamer_mp3_player::GStreamerMp3Player, mp3_player::Mp3Player};

// TODO : Add concept of InitializedBusinessCore which is returned by BusinessCore::initialize, and consumes self
pub struct BusinessCore {
    application_dir_initializer: ApplicationDirInitializer,
    rss_provider: RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    pub player: Arc<TokioMutex<dyn Mp3Player + Send>>,
    pub podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    path_provider: Rc<dyn PathProvider>,
    notifications_sender: Sender<Notification>,
}

impl BusinessCore {
    pub fn new(
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        path_provider: Rc<dyn PathProvider>,
        notifications_sender: Sender<Notification>,
    ) -> BusinessCore {
        let podcast_library = Arc::new(TokioMutex::new(PodcastLibrary::new()));
        BusinessCore {
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
        }
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
        // TODO : Prevent adding an url several times
        self.rss_provider.add_url(url)?;
        info!("Url added successfully");
        self.send_notification(format!("Url {} added successfully", url))
            .await;
        Ok(())
    }

    pub async fn load_feed(&mut self, url: &str) -> Result<(), ()> {
        let feeds = self.rss_provider.get_all_feeds().await;
        let channel = feeds.iter().find(|c| c.0 == url);
        if channel.is_none() {
            error!("Could not find channel matching URL {}", url);
            return Err(());
        }
        let podcast = self.podcast_builder.build(&channel.unwrap().1);
        self.podcast_library.lock().await.push(podcast);
        Ok(())
    }

    pub async fn build_podcasts(&mut self) {
        let channels = self.rss_provider.get_all_feeds().await;
        let mut podcasts: Vec<Podcast> = vec![];
        for channel in &channels {
            podcasts.push(self.podcast_builder.build(&channel.1))
        }
        self.podcast_library.lock().await.push(podcasts);
        self.send_notification("Building library done".to_string())
            .await;
    }

    pub async fn download_episode(&mut self, episode: &PodcastEpisode) -> Result<(), ()> {
        self.send_notification(format!("Downloading \"{}\"", episode.title))
            .await;
        if (self.podcast_downloader.download_episode(episode).await).is_err() {
            self.send_notification("Downloading failed".to_string())
                .await;
            return Err(());
        }
        self.send_notification("Downloading successful".to_string())
            .await;

        Ok(())
    }

    async fn send_notification(&mut self, notification: Notification) {
        self.notifications_sender
            .send(notification)
            .await
            .expect("Writing notification in channel failed");
    }
}
