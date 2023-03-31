use std::future::Future;
use std::pin::Pin;
use std::task::Poll::Ready;

use command_management::commands::command_enum::Command;
use data_transport::DataSender;
use podcast_management::data_objects::hashable::Hashable;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub type CallbackReturnType = Result<(), ()>;
pub type AsyncCallbackReturnType = Pin<Box<(dyn Future<Output = CallbackReturnType>)>>;
pub type BuildCommandCallback<'a, T> = dyn Fn(&'a T) -> String;
//pub type SendCommandCallback = dyn Fn(String, &mut DataSender<String>) -> AsyncCallbackReturnType;

pub fn __build_play_command<'a>(
    episode: &'a PodcastEpisode,
    sender: &'a mut DataSender<String>,
) -> Pin<Box<(dyn Future<Output = Result<(), ()>> + 'a)>> {
    let play_command = Command::Play(None).to_string();
    let command = format!("{} {}", play_command, episode.hash());
    Box::pin(sender.send(command))
}

pub fn build_play_command<'a>(episode: &'a PodcastEpisode) -> String {
    let play_command = Command::Play(None).to_string();
    format!("{} {}", play_command, episode.hash())
}

//pub fn send<'a>(command: &str, sender: &'a mut DataSender<String>) -> AsyncCallbackReturnType {
//    Box::pin(sender.send(command.to_owned()))
//}

pub fn send(
    command: String,
    sender: &mut DataSender<String>,
) -> Pin<Box<dyn Future<Output = Result<(), ()>> + '_>> {
    //AsyncCallbackReturnType {
    let f = sender.send(command);
    Box::pin(f)
}
