use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
use podcast_player::mp3_player::Mp3Player;
use tokio::io::AsyncBufReadExt;

pub struct BusinessCore {
    application_dir_initializer: ApplicationDirInitializer,
    rss_provider: RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    player: Arc<Mutex<Mp3Player>>,
    podcast_library: Arc<Mutex<PodcastLibrary>>,
    path_provider: DefaultPathProvider,
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        let path_provider = DefaultPathProvider {};
        let mp3_player = Arc::new(Mutex::new(Mp3Player::new()));
        let podcast_library = Arc::new(Mutex::new(PodcastLibrary::new()));
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
        self.podcast_library.lock().unwrap().push(&mut podcasts);
    }

    pub async fn download_some_random_podcast(&mut self) -> Result<(), ()> {
        // TODO : Remove me
        if (self
            .podcast_downloader
            .download_episode(&self.podcast_library.lock().unwrap().podcasts[0].episodes[0])
            .await)
            .is_err()
        {
            return Err(());
        }
        println!("Podcast downloaded lul");

        let path = self
            .path_provider
            .compute_episode_path(&self.podcast_library.lock().unwrap().podcasts[0].episodes[0])
            .into_os_string()
            .into_string()
            .unwrap();
        {
            self.player.lock().unwrap().play_file(&path).unwrap();
        }
        println!("Podcast played lul");

        let t_stdin = tokio::io::stdin();
        let mut reader = tokio::io::BufReader::new(t_stdin);
        let mut line: String = String::from("");
        let mut size_read = reader.read_line(&mut line).await.unwrap();
        while size_read > 1 {
            println!("YAS {size_read}");
            size_read = reader.read_line(&mut line).await.unwrap();
        }

        Ok(())
    }
}

impl Default for BusinessCore {
    fn default() -> Self {
        Self::new()
    }
}
