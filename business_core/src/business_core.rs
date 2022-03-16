use rss_management::application_dir_initializer::ApplicationDirInitializer;
use std::path::Path;
use std::path::PathBuf;
pub struct BusinessCore {
    application_dir_initializer : ApplicationDirInitializer
}

impl BusinessCore {
    pub fn new() -> BusinessCore {
        BusinessCore{ application_dir_initializer : ApplicationDirInitializer {} }
    }

    pub fn initialize(&self) {
        let app_dir_path = ApplicationDirInitializer::default_app_dir_path();
        let app_dir_path = app_dir_path.to_str().unwrap();
        if ! ApplicationDirInitializer::is_app_dir(PathBuf::from(app_dir_path)) {
            ApplicationDirInitializer::initialize_application_dir(&app_dir_path).expect("Application dir initialization failed");
        }
    }
}