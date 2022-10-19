use super::command_enum::Command;
use crate::command_error::{CommandError, ErrorKind};
use std::collections::HashMap;
use std::i64;

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

fn is_hash(hash: &str) -> bool {
    i64::from_str_radix(hash, 16).is_ok()
}

pub fn build_select_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    // TODO : Return Err(CommandError) when the hash parsing fails
    assert_eq!(parameters.len(), 1);
    assert_eq!(parameters[0].len(), HASH_LEN);
    let hash = &parameters[0];
    assert!(is_hash(hash));
    Ok(Command::Select(hash.to_string()))
}

pub fn build_add_rss_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    assert_eq!(parameters.len(), 1);

    let url = Url::parse(&parameters[0]);
    if let Err(_) = url {
        return Err(CommandError::new(
            None,
            ErrorKind::ParameterParsingFailed,
            Some("add_url".to_string()),
            Some("Url parsing failed".to_string()),
        ));
    }

    Ok(Command::AddRss(url.unwrap()))
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
