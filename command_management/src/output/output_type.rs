use crate::commands::command_enum::Command;
use crate::commands::helps::command_help::CommandHelp;
use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

#[derive(Clone)]
pub enum OutputType {
    None,
    Podcasts(Vec<Podcast>),
    Episodes(Vec<PodcastEpisode>),
    CommandHelps(Vec<CommandHelp>),
    RawString(String),
}

impl PartialEq for OutputType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Podcasts(_), Self::Podcasts(_))
                | (Self::Episodes(_), Self::Episodes(_))
                | (Self::CommandHelps(_), Self::CommandHelps(_))
                | (Self::RawString(_), Self::RawString(_))
                | (Self::None, Self::None)
        )
    }
}
