use std::path::PathBuf;

use home::home_dir;

pub use super::path_provider::PathProvider;

#[derive(Copy, Clone)]
pub struct DefaultPathProvider {}

impl PathProvider for DefaultPathProvider {
    fn app_dir_path(&self) -> PathBuf {
        let home_dir_path = home_dir().unwrap();
        let home_dir_path: &str = home_dir_path.to_str().unwrap();
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
    fn first_start_marker_file_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(PathBuf::from(self.first_start_marker_file_name()));
        p
    }
    fn rss_feed_list_file_name(&self) -> &'static str {
        "rss_feed_list"
    }

    fn download_dir_name(&self) -> &'static str {
        "downloads"
    }
    fn first_start_marker_file_name(&self) -> &'static str {
        "first_start"
    }
    fn podcast_progresses_dir_name(&self) -> &'static str {
        "podcast_progresses"
    }
    fn podcast_progresses_dir_path(&self) -> PathBuf {
        let mut p = self.app_dir_path();
        p.push(self.podcast_progresses_dir_name());
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
