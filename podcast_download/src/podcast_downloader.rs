use std::{path::PathBuf, str::FromStr};

use bytes::Bytes;
use log::info;
use reqwest;

use fs_utils::write_utils::write_bytes_in_file;

use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct PodcastDownloader {
    download_dir: PathBuf,
    client: reqwest::Client,
}

impl PodcastDownloader {
    pub fn new(download_dir: &str) -> PodcastDownloader {
        PodcastDownloader {
            download_dir: PathBuf::from_str(download_dir).unwrap(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn download_episode(
        &self,
        episode: &PodcastEpisode,
    ) -> Result<PathBuf, std::io::Error> {
        let url = &episode.url;
        let request = self.client.get(url);
        info!(
            "Downloading podcast episode {episode_title}",
            episode_title = episode.title
        );

        let mut response: reqwest::Response;
        match request.send().await {
            Ok(r) => response = r,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Response reception failed",
                ))
            }
        }

        let mut result: Bytes;
        match response.bytes().await {
            Ok(b) => result = b,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Conversion to bytes failed",
                ))
            }
        }
        let mut output_path = self.download_dir.clone();
        output_path.push(&episode.title);
        write_bytes_in_file(
            &output_path.clone().into_os_string().to_str().unwrap(),
            &result,
        )?;

        info!(
            "Download of podcast episode {episode_title} finished",
            episode_title = episode.title
        );
        Ok(output_path)
    }
}
