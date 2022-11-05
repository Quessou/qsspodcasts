use super::command_enum::Command;
use super::duration_utils::string_to_duration;
use super::hash_utils::is_hash;
use crate::command_error::{CommandError, ErrorKind};
use std::collections::HashMap;
use url::Url;

const HASH_LEN: usize = 6;

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

pub fn build_select_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(CommandError::new(
            None,
            ErrorKind::BadParameterCount,
            Some("select".to_string()),
            Some("Bad parameter count".to_string()),
        ));
    }

    let hash = &parameters[0];
    if hash.len() != HASH_LEN || !is_hash(hash) {
        return Err(CommandError::new(
            None,
            ErrorKind::ParameterParsingFailed,
            Some("select".to_string()),
            Some("Parameter parsing failed".to_string()),
        ));
    }
    Ok(Command::Select(hash.to_string()))
}

pub fn build_add_rss_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(CommandError::new(
            None,
            ErrorKind::BadParameterCount,
            Some("add_rss".to_string()),
            Some("Bad parameter count".to_string()),
        ));
    }

    let url = Url::parse(&parameters[0]);
    if url.is_err() {
        return Err(CommandError::new(
            None,
            ErrorKind::ParameterParsingFailed,
            Some("add_url".to_string()),
            Some("Url parsing failed".to_string()),
        ));
    }

    Ok(Command::AddRss(url.unwrap()))
}

pub fn build_advance_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(CommandError::new(
            None,
            ErrorKind::BadParameterCount,
            Some("advance".to_string()),
            Some("Bad parameter count".to_string()),
        ));
    }

    match string_to_duration(&parameters[0]) {
        Ok(o) => Ok(Command::Advance(o)),
        Err(_) => Err(CommandError::new(
            None,
            ErrorKind::ParameterParsingFailed,
            Some("advance".to_string()),
            Some("duration parsing failed".to_string()),
        )),
    }
}

pub fn build_go_back_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(CommandError::new(
            None,
            ErrorKind::BadParameterCount,
            Some("go_back".to_string()),
            Some("Bad parameter count".to_string()),
        ));
    }

    match string_to_duration(&parameters[0]) {
        Ok(o) => Ok(Command::GoBack(o)),
        Err(_) => Err(CommandError::new(
            None,
            ErrorKind::ParameterParsingFailed,
            Some("go_back".to_string()),
            Some("duration parsing failed".to_string()),
        )),
    }
}

pub fn get_factory_hashmap() -> HashMap<&'static str, FactoryFn> {
    let mut factory_hashmap: HashMap<&'static str, FactoryFn> = HashMap::new();
    factory_hashmap.insert("play", build_play_command);
    factory_hashmap.insert("pause", build_pause_command);
    factory_hashmap.insert("exit", build_exit_command);
    factory_hashmap.insert("list_podcasts", build_list_podcasts_command);
    factory_hashmap.insert("list_episodes", build_list_episodes_command);
    factory_hashmap.insert("select", build_select_command);
    factory_hashmap.insert("add_rss", build_add_rss_command);
    factory_hashmap.insert("advance", build_advance_command);
    factory_hashmap.insert("go_back", build_go_back_command);
    factory_hashmap
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("1" => true; "basic base 10 case")]
    #[test_case("1abf" => true; "basic base 16 case")]
    #[test_case("1abfg" => false; "base 16 invalid case")]
    fn test_is_hash(hash: &str) -> bool {
        is_hash(hash)
    }
}
