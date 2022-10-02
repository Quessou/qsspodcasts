use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

pub enum OutputType {
    Podcasts(Vec<Podcast>),
    Episodes(Vec<PodcastEpisode>),
    RawString(String),
}
