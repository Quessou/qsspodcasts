use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

#[derive(Clone)]
pub enum OutputType {
    Podcasts(Vec<Podcast>),
    Episodes(Vec<PodcastEpisode>),
    RawString(String),
}

impl PartialEq for OutputType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Podcasts(_), Self::Podcasts(_))
                | (Self::Episodes(_), Self::Episodes(_))
                | (Self::RawString(_), Self::RawString(_))
        )
    }
}
