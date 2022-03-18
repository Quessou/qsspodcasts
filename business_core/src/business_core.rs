use rss_management::{application_dir_initializer::ApplicationDirInitializer, rss_provider::RssProvider};
use std::path::PathBuf;
use std::io::Error as IoError;
pub struct BusinessCore {
    application_dir_initializer : ApplicationDirInitializer,
    rss_provider : RssProvider
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        BusinessCore{ application_dir_initializer : ApplicationDirInitializer {}, 
        rss_provider : RssProvider::new(ApplicationDirInitializer::default_rss_feed_list_file_path().to_str()) }
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