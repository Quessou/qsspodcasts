use std::future::Future;
use std::pin::Pin;

use super::modal_action_data::ModalActionData;
use command_management::commands::command_enum::Command;
use data_transport::DataSender;
use podcast_management::data_objects::hashable::Hashable;
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

pub fn play<'a>(
    episode: &'a PodcastEpisode,
    sender: &'a mut DataSender<String>,
) -> Pin<Box<(dyn Future<Output = Result<(), ()>> + 'a)>> {
    let play_command = Command::Play(None).to_string();
    let command = format!("{} {}", play_command, episode.hash());
    Box::pin(sender.send(command))
}

impl ModalActionable for PodcastEpisode {
    fn get_action_list(&self) -> Vec<ModalActionData<Self>>
    where
        Self: Sized,
    {
        vec![
            //ModalActionData {
            //    action: "select".to_owned(),
            //    actionable: self,
            //},
            ModalActionData::new("play".to_owned(), self, Box::new(play)),
        ]
    }
}
