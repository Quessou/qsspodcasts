use super::modal_action_callbacks::*;
use super::modal_action_data::ModalActionData;
use command_management::commands::helps::command_help::CommandHelp;
use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

pub trait ModalActionable {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized;
}

impl ModalActionable for Podcast {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![ModalActionData::new(
            "List episodes".to_owned(),
            self,
            Box::new(build_list_episodes_command),
        )]
    }
}

impl ModalActionable for PodcastEpisode {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![ModalActionData::new(
            "Play".to_owned(),
            self,
            Box::new(build_play_command),
        )]
    }
}

impl ModalActionable for CommandHelp {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![]
    }
}
