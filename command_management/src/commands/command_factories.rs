use crate::command_error::CommandError;

use super::command_enum::Command;

use std::collections::HashMap;

pub type FactoryFn = fn(Vec<String>) -> Result<Command, CommandError>;

pub fn build_play_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::Play)
}

pub fn build_pause_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::Pause)
}

pub fn build_exit_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::Exit)
}

pub fn build_list_podcasts_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::ListPodcasts)
}

pub fn build_list_episodes_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::ListEpisodes)
}

pub fn get_factory_hashmap() -> HashMap<String, FactoryFn> {
    let mut factory_hashmap: HashMap<String, FactoryFn> = HashMap::new();
    factory_hashmap.insert("play".to_string(), build_play_command);
    factory_hashmap.insert("pause".to_string(), build_pause_command);
    factory_hashmap.insert("exit".to_string(), build_exit_command);
    factory_hashmap.insert("list_podcasts".to_string(), build_list_podcasts_command);
    factory_hashmap.insert("list_episodes".to_string(), build_list_episodes_command);
    factory_hashmap
}
