pub use std::path::PathBuf;

pub use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub trait PathProvider {
    fn app_dir_path(&self) -> PathBuf;
    fn rss_feed_list_file_path(&self) -> PathBuf;
    fn download_dir_path(&self) -> PathBuf;
    fn rss_feed_list_file_name(&self) -> &'static str;
    fn download_dir_name(&self) -> &'static str;
    fn podcast_progresses_dir_name(&self) -> &'static str;
    fn podcast_progresses_dir_path(&self) -> PathBuf;

    fn compute_episode_path(&self, episode: &PodcastEpisode) -> PathBuf {
        let mut download_dir: PathBuf = self.download_dir_path();
        download_dir.push(PathBuf::from(episode.get_file_name().replace(' ', "_")));
        download_dir
    }
    fn first_start_marker_file_name(&self) -> &'static str;
    fn first_start_marker_file_path(&self) -> PathBuf;
}
