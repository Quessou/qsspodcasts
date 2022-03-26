use rss;
use super::episode_builder::EpisodeBuilder;
use crate::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

pub struct PodcastBuilder {
    episode_builder : EpisodeBuilder
}

impl PodcastBuilder {
    pub fn new() -> PodcastBuilder {
        PodcastBuilder { episode_builder : EpisodeBuilder {}}
    }

    pub fn build(&self, channel : rss::Channel) -> Podcast {
        let episodes = channel.items().iter().map(|i| self.episode_builder.build(i).unwrap()).collect::<Vec<PodcastEpisode>>();
        Podcast::new(&channel.title, &channel.link, &channel.description, channel.copyright, channel.pub_date, channel.image, episodes)
    }
}