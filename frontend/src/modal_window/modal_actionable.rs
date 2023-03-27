use super::modal_action_data::ModalActionData;
use podcast_management::data_objects::{podcast::Podcast, podcast_episode::PodcastEpisode};

pub(crate) trait ModalActionable {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized;
}

impl ModalActionable for Podcast {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![]
    }
}

impl ModalActionable for PodcastEpisode {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![
            ModalActionData {
                action: "select".to_owned(),
                actionable: self,
            },
            ModalActionData {
                action: "play".to_owned(),
                actionable: self,
            },
        ]
    }
}
