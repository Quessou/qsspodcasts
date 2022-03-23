use rss_management::{local_storage::{application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider}, url_storer::file_url_storer::{FileUrlStorer}};
use podcast_management::builders::podcast_builder::PodcastBuilder;
use std::path::PathBuf;
use std::io::Error as IoError;
pub struct BusinessCore {
    _application_dir_initializer : ApplicationDirInitializer,
    rss_provider : RssProvider<FileUrlStorer>,
    podcast_builder: PodcastBuilder
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        BusinessCore{ _application_dir_initializer : ApplicationDirInitializer {},
        rss_provider : RssProvider::new(FileUrlStorer::new(PathBuf::from(ApplicationDirInitializer::default_rss_feed_list_file_path().to_str().unwrap())) ),
        podcast_builder : PodcastBuilder::new() }
    }

    pub fn initialize(&self) {
        let app_dir_path = ApplicationDirInitializer::default_app_dir_path();
        let app_dir_path = app_dir_path.to_str().unwrap();
        if ! ApplicationDirInitializer::is_app_dir(PathBuf::from(app_dir_path)) {
            ApplicationDirInitializer::initialize_application_dir(&app_dir_path).expect("Application dir initialization failed");
        }
    }

    pub fn add_url(&mut self, url: &str) -> Result<(), IoError> {
        self.rss_provider.add_url(url)?;
        Ok(())
    }
}