use super::episode_builder::EpisodeBuilder;
use crate::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};
use rss;

pub struct PodcastBuilder {
    episode_builder: EpisodeBuilder,
}

impl PodcastBuilder {
    pub fn new() -> PodcastBuilder {
        PodcastBuilder {
            episode_builder: EpisodeBuilder {},
        }
    }

    pub fn build(&self, channel: &rss::Channel) -> Podcast {
        let episodes = channel
            .items()
            .iter()
            .map(|i| self.episode_builder.build(i, &channel.title).unwrap())
            .collect::<Vec<PodcastEpisode>>();
        Podcast::new(
            &channel.title,
            &channel.link,
            &channel.description,
            channel.copyright.clone(),
            channel.pub_date.clone(),
            channel.image.clone(),
            episodes,
        )
    }
}

impl Default for PodcastBuilder {
    fn default() -> Self {
        Self::new()
    }
}
