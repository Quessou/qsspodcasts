use std::path::PathBuf;

use bytes::Bytes;
use log::{debug, info};
use reqwest;

use fs_utils::write_utils::write_bytes_in_file;

use podcast_management::data_objects::podcast_episode::PodcastEpisode;

use path_providing::path_provider::PathProvider;

pub struct PodcastDownloader {
    client: reqwest::Client,
    path_provider: Box<dyn PathProvider>,
}

impl PodcastDownloader {
    pub fn new(path_provider: Box<dyn PathProvider>) -> PodcastDownloader {
        PodcastDownloader {
            client: reqwest::Client::new(),
            path_provider,
        }
    }

    pub async fn download_episode(
        &self,
        episode: &PodcastEpisode,
    ) -> Result<PathBuf, std::io::Error> {
        let url = &episode.url;
        let request = self.client.get(url);
        let output_path = self.path_provider.compute_episode_path(episode);

        if output_path.exists() {
            debug!("Episode already downloaded, not doing anything");
            return Ok(output_path);
        }

        info!(
            "Downloading podcast episode {episode_title}",
            episode_title = episode.title
        );

        let response: reqwest::Response = match request.send().await {
            Ok(r) => r,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Response reception failed",
                ))
            }
        };

        let result: Bytes = match response.bytes().await {
            Ok(b) => b,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Conversion to bytes failed",
                ))
            }
        };

        write_bytes_in_file(
            output_path.clone().into_os_string().to_str().unwrap(),
            &result,
        )?;

        info!(
            "Download of podcast episode {episode_title} finished",
            episode_title = episode.title
        );
        Ok(output_path)
    }
}
