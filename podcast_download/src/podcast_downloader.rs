use std::{path::PathBuf, str::FromStr};

use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct PodcastDownloader {
    download_dir: PathBuf
}

impl PodcastDownloader {
    pub fn new(download_dir: &str) -> PodcastDownloader {
        PodcastDownloader { download_dir: PathBuf::from_str(download_dir).unwrap() }
    }

    pub fn download_episode(&self, episode: PodcastEpisode) {
        
    }
}