use std::{path::PathBuf, str::FromStr};

use reqwest;
use log::info;
use bytes::Bytes;

use fs_utils::write_utils::write_bytes_in_file;

use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct PodcastDownloader {
    download_dir: PathBuf,
    client: reqwest::Client
}

impl PodcastDownloader {
    pub fn new(download_dir: &str) -> PodcastDownloader {
        PodcastDownloader { download_dir: PathBuf::from_str(download_dir).unwrap(), client: reqwest::Client::new() }
    }

    pub async fn download_episode(&self, episode: &PodcastEpisode) -> Result<(), reqwest::Error> {
        let url = &episode.url;
        let request = self.client.get(url);
        info!("Downloading podcast episode {episode_title}", episode_title=episode.title);
        let result: Bytes = request.send().await?.bytes().await?;
        write_bytes_in_file("/tmp/toto.mp3", &result);
        info!("Download of podcast episode {episode_title} finished", episode_title=episode.title);
        Ok(())
    }
}