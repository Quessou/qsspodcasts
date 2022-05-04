use std::path::PathBuf;

pub use super::path_provider::PathProvider;

#[derive(Clone)]
pub struct DummyPathProvider {
    test_root_path: String,
}

impl DummyPathProvider {
    pub fn new(root_path: &str) -> DummyPathProvider {
        DummyPathProvider {
            test_root_path: root_path.to_string(),
        }
    }
}

impl PathProvider for DummyPathProvider {
    fn app_dir_path(&self) -> PathBuf {
        let home_dir_path: &str = self.test_root_path.as_str();
        [home_dir_path, ".qsspodcasts"].iter().collect()
    }
    fn rss_feed_list_file_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(PathBuf::from(self.rss_feed_list_file_name()));
        p
    }
    fn download_dir_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(PathBuf::from(self.download_dir_name()));
        p
    }
    fn rss_feed_list_file_name(&self) -> &'static str {
        "test_rss_feed_list"
    }

    fn download_dir_name(&self) -> &'static str {
        "test_downloads"
    }
}
