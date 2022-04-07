use std::{path::PathBuf, str::FromStr};
use reqwest;

use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct PodcastDownloader {
    download_dir: PathBuf,
    client: reqwest::Client
}

impl PodcastDownloader {
    pub fn new(download_dir: &str) -> PodcastDownloader {
        PodcastDownloader { download_dir: PathBuf::from_str(download_dir).unwrap(), client: reqwest::Client::new() }
    }

    pub async fn download_episode(&self, episode: PodcastEpisode) -> Result<(), reqwest::Error> {
        let url = episode.url;
        let request = self.client.get(url);
        request.send().await?;
        Ok(())
    }
}