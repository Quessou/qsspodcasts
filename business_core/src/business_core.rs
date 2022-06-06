use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use log::error;

use rss_management::{
    local_storage::{
        application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider,
    },
    url_storage::file_url_storer::FileUrlStorer,
};

use path_providing::default_path_provider::{DefaultPathProvider, PathProvider};
use podcast_download::podcast_downloader::PodcastDownloader;
use podcast_management::{
    builders::podcast_builder::PodcastBuilder, data_objects::podcast::Podcast,
    podcast_library::PodcastLibrary,
};
use podcast_player::players::{
    gstreamer_mp3_player::GStreamerMp3Player, mp3_player::Mp3Player,
    rodio_mp3_player::RodioMp3Player,
};

pub struct BusinessCore {
    application_dir_initializer: ApplicationDirInitializer,
    rss_provider: RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    pub player: Arc<TokioMutex<dyn Mp3Player + Send>>,
    pub podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    path_provider: DefaultPathProvider,
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        let path_provider = DefaultPathProvider {};
        let mp3_player = Arc::new(TokioMutex::new(GStreamerMp3Player::new(Box::new(
            path_provider,
        ))));
        let podcast_library = Arc::new(TokioMutex::new(PodcastLibrary::new()));
        BusinessCore {
            rss_provider: RssProvider::new(FileUrlStorer::new(PathBuf::from(
                path_provider.rss_feed_list_file_path().to_str().unwrap(),
            ))),
            podcast_builder: PodcastBuilder::new(),
            podcast_downloader: PodcastDownloader::new(Box::new(path_provider)),
            player: mp3_player,
            podcast_library,
            application_dir_initializer: ApplicationDirInitializer {
                path_provider: Box::new(path_provider),
            },
            path_provider,
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

    pub fn add_url(&mut self, url: &str) -> Result<(), IoError> {
        self.rss_provider.add_url(url)?;
        Ok(())
    }

    pub async fn build_podcasts(&mut self) {
        let channels = self.rss_provider.get_all_feeds().await;
        let mut podcasts: Vec<Podcast> = vec![];
        for channel in &channels {
            podcasts.push(self.podcast_builder.build(channel))
        }
        self.podcast_library.lock().await.push(&mut podcasts);
    }

    pub async fn download_some_random_podcast(&mut self) -> Result<(), ()> {
        // TODO : Remove me
        let episode = &self.podcast_library.lock().await.podcasts[0].episodes[0];
        if (self.podcast_downloader.download_episode(episode).await).is_err() {
            return Err(());
        }

        if self.player.lock().await.select_episode(episode).is_err() {
            error!("Selecting of episode failed");
            return Err(());
        }
        if self.player.lock().await.play_selected_episode().is_err() {
            error!("Playing of episode failed");
            return Err(());
        }

        Ok(())
    }
}

impl Default for BusinessCore {
    fn default() -> Self {
        Self::new()
    }
}
