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
    fn podcast_progresses_dir_name(&self) -> &'static str {
        "podcast_progresses"
    }

    fn podcast_progresses_dir_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(self.podcast_progresses_dir_name());
        p
    }

    fn rss_feed_list_file_name(&self) -> &'static str {
        "test_rss_feed_list"
    }

    fn download_dir_name(&self) -> &'static str {
        "test_downloads"
    }

    fn first_start_marker_file_name(&self) -> &'static str {
        "first_start"
    }

    fn first_start_marker_file_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(PathBuf::from(self.first_start_marker_file_name()));
        p
    }

    fn podcast_progress_file_path(&self, hash: &str) -> PathBuf {
        let mut p = self.podcast_progresses_dir_path();
        p.push(hash);
        p
    }
    fn finished_podcasts_dir_name(&self) -> &'static str {
        "finished_podcasts"
    }

    fn finished_podcasts_dir_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(self.finished_podcasts_dir_name());
        p
    }

    fn compute_finished_podcast_file_path(&self, hash: &str) -> PathBuf {
        let mut p = self.finished_podcasts_dir_path();
        p.push(hash);
        p
    }
}
