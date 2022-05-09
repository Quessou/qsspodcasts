use std::io::Error as IoError;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rss_management::{
    local_storage::{
        application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider,
    },
    url_storer::file_url_storer::FileUrlStorer,
};

use path_providing::default_path_provider::{DefaultPathProvider, PathProvider};
use podcast_download::podcast_downloader::PodcastDownloader;
use podcast_management::{
    builders::podcast_builder::PodcastBuilder, data_objects::podcast::Podcast,
    podcast_library::PodcastLibrary,
};
use podcast_player::mp3_player::Mp3Player;
// use command_management::command_engine::CommandEngine;

pub struct BusinessCore {
    application_dir_initializer: ApplicationDirInitializer,
    rss_provider: RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder,
    podcast_downloader: PodcastDownloader,
    player: Arc<Mutex<Mp3Player>>,
    podcast_library: Arc<Mutex<PodcastLibrary>>,
    path_provider: DefaultPathProvider,
    //command_engine: Arc<Mutex<CommandEngine>>
}

fn add(i: u32, j: u32) -> u32 {
    let toto = i + 10;
    i + j + toto
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        let t = add(1, 20);
        let path_provider = DefaultPathProvider {};
        let mp3_player = Arc::new(Mutex::new(Mp3Player::new()));
        let podcast_library = Arc::new(Mutex::new(PodcastLibrary::new()));
        //  let command_engine = Arc::new(Mutex::new(CommandEngine::new(mp3_player.clone(), podcast_library.clone())));
        BusinessCore {
            rss_provider: RssProvider::new(FileUrlStorer::new(PathBuf::from(
                path_provider.rss_feed_list_file_path().to_str().unwrap(),
            ))),
            podcast_builder: PodcastBuilder::new(),
            podcast_downloader: PodcastDownloader::new(
                Box::new(path_provider.clone())
            ),
            player: mp3_player.clone(),
            podcast_library: podcast_library.clone(),
            application_dir_initializer: ApplicationDirInitializer {
                path_provider: Box::new(path_provider.clone()),
            },
            path_provider,
            //    command_engine: command_engine,
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
                .initialize_application_dir(&app_dir_path)
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
        if let Err(_) = self
            .podcast_downloader
            .download_episode(&self.podcast_library.lock().unwrap().podcasts[0].episodes[0])
            .await
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
        self.player.lock().unwrap().play_file(&path);

        println!("Podcast played lul");
        Ok(())
    }

    //pub async fn run(&mut self) {
    //    CommandEngine::run(self.command_engine.clone());
    //}
}
