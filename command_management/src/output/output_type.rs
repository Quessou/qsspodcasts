use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

#[derive(Clone)]
pub enum OutputType {
    Podcasts(Vec<Podcast>),
    Episodes(Vec<PodcastEpisode>),
    RawString(String),
}
