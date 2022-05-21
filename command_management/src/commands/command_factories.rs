use super::command_enum::Command;

use std::collections::HashMap;

pub type FactoryFn = fn() -> Command;

pub fn build_play_command() -> Command {
    Command::Play
}

pub fn build_pause_command() -> Command {
    Command::Pause
}

pub fn get_factory_hashmap() -> HashMap<String, FactoryFn> {
    let mut factory_hashmap: HashMap<String, FactoryFn> = HashMap::new();
    factory_hashmap.insert("play".to_string(), build_play_command);
    factory_hashmap.insert("pause".to_string(), build_pause_command);
    factory_hashmap
}
