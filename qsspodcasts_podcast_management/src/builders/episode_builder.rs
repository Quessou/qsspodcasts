use crate::data_objects::podcast_episode::PodcastEpisode;
use rss;

pub struct EpisodeBuilder {}

impl EpisodeBuilder {
    pub fn build(&self, item: &rss::Item, podcast_name: &str) -> Result<PodcastEpisode, String> {
        if let Some(mut episode) = PodcastEpisode::from_item(item) {
            episode.set_podcast_name(podcast_name);
            return Ok(episode);
        }
        Err("Episode could not be built".to_owned())
    }
}
