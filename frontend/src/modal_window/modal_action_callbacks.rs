use command_management::commands::command_enum::Command;
use podcast_management::data_objects::hashable::Hashable;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub type CallbackReturnType = Result<(), ()>;
pub type BuildCommandCallback<'a, T> = dyn Fn(&'a T) -> String;

pub fn build_play_command(episode: &PodcastEpisode) -> String {
    let play_command = Command::Play(None).to_string();
    format!("{} {}", play_command, episode.hash())
}
