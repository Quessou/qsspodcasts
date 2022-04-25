use std::path::PathBuf;
use std::io::Error as IoError;

use rss_management::{local_storage::{application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider}, url_storer::file_url_storer::{FileUrlStorer}};

use podcast_management::{builders::podcast_builder::PodcastBuilder, data_objects::{podcast::Podcast,podcast_episode::PodcastEpisode}};
use podcast_download::podcast_downloader::PodcastDownloader;
use podcast_player::mp3_player::Mp3Player;
use path_providing::default_path_provider::{PathProvider, DefaultPathProvider};

pub struct BusinessCore {
    application_dir_initializer : ApplicationDirInitializer,
    rss_provider : RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    player: Mp3Player,
    podcasts: Vec<Podcast>,
    path_provider: DefaultPathProvider
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        let path_provider = DefaultPathProvider{};
        BusinessCore {
            rss_provider : RssProvider::new(FileUrlStorer::new(PathBuf::from(path_provider.rss_feed_list_file_path().to_str().unwrap())) ),
            podcast_builder : PodcastBuilder::new(),
            podcast_downloader: PodcastDownloader::new(path_provider.download_dir_path().to_str().unwrap()),
            player: Mp3Player{},
            podcasts: vec![],
            application_dir_initializer : ApplicationDirInitializer {path_provider: Box::new(path_provider.clone()) },
            path_provider: path_provider,
        }
    }

    pub fn initialize(&self) {
        let app_dir_path = self.path_provider.app_dir_path();
        let app_dir_path = app_dir_path.to_str().unwrap();
        if ! self.application_dir_initializer.is_app_dir_created(PathBuf::from(app_dir_path)) {
            self.application_dir_initializer.initialize_application_dir(&app_dir_path).expect("Application dir initialization failed");
        }
    }

    pub fn add_url(&mut self, url: &str) -> Result<(), IoError> {
        self.rss_provider.add_url(url)?;
        Ok(())
    }

    pub async fn build_podcasts(&mut self) {
        let channels = self.rss_provider.get_all_feeds().await;
        for channel in &channels {
            self.podcasts.push(self.podcast_builder.build(channel))
        }
    }

    pub async fn download_some_random_podcast(&mut self) -> Result<(), ()> {
        println!("Entering");
        if let Err(e) = self.podcast_downloader.download_episode(&self.podcasts[0].episodes[0]).await {
            return Err(());
        }

        let path = self.path_provider.compute_episode_path(&self.podcasts[0].episodes[0]).into_os_string().into_string().unwrap();
        self.player.play_file(&path);
        Ok(())
    }
}