use super::command_enum::{Command, CommandDuration, CommandUrl};
use super::duration_utils::string_to_duration;
use super::hash_utils::is_hash;
use crate::command_error::{CommandError, ErrorKind};
use std::collections::HashMap;
use url::Url;

const HASH_LEN: usize = 6;

pub type FactoryFn = fn(Vec<String>) -> Result<Command, CommandError>;

fn build_bad_parameter_count_error(command_name: &str) -> CommandError {
    CommandError::new(
        None,
        ErrorKind::BadParameterCount,
        Some(command_name.to_string()),
        Some("Bad parameter count".to_string()),
    )
}

fn build_parsing_failed_error(command_name: &str, error_text: &str) -> CommandError {
    CommandError::new(
        None,
        ErrorKind::ParameterParsingFailed,
        Some(command_name.to_string()),
        Some(error_text.to_string()),
    )
}

pub fn build_play_command(mut parameters: Vec<String>) -> Result<Command, CommandError> {
    let command = if !parameters.is_empty() && is_hash(&parameters[0]) {
        let hash = parameters.pop().unwrap();
        Command::Play(Some(hash))
    } else if !parameters.is_empty() && !is_hash(&parameters[0]) {
        let error = build_parsing_failed_error("Play", "Parsing of hash failed");
        return Err(error);
    } else {
        Command::Play(None)
    };
    Ok(command)
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
        return Err(build_bad_parameter_count_error("select"));
    }

    let hash = &parameters[0];
    if hash.len() != HASH_LEN || !is_hash(hash) {
        return Err(build_parsing_failed_error(
            "select",
            "Parameter parsing failed",
        ));
    }
    Ok(Command::Select(hash.to_string()))
}

pub fn build_add_rss_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("add_rss"));
    }

    let url = Url::parse(&parameters[0]);
    if url.is_err() {
        return Err(build_parsing_failed_error("add_url", "Url parsing failed"));
    }

    Ok(Command::AddRss(CommandUrl(url.unwrap())))
}

pub fn build_advance_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("advance"));
    }

    match string_to_duration(&parameters[0]) {
        Ok(o) => Ok(Command::Advance(CommandDuration(o))),
        Err(_) => Err(build_parsing_failed_error(
            "advance",
            "duration parsing failed",
        )),
    }
}

pub fn build_go_back_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("go_back"));
    }

    match string_to_duration(&parameters[0]) {
        Ok(o) => Ok(Command::GoBack(CommandDuration(o))),
        Err(_) => Err(build_parsing_failed_error(
            "go_back",
            "duration parsing failed",
        )),
    }
}

pub fn build_help_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.is_empty() {
        return Ok(Command::Help(None));
    } else if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("help"));
    }
    Ok(Command::Help(Some(parameters[0].clone())))
}

pub fn get_factory_hashmap() -> HashMap<String, FactoryFn> {
    let mut factory_hashmap: HashMap<String, FactoryFn> = HashMap::new();
    factory_hashmap.insert(Command::Play(None).to_string(), build_play_command);
    factory_hashmap.insert(Command::Pause.to_string(), build_pause_command);
    factory_hashmap.insert(Command::Exit.to_string(), build_exit_command);
    factory_hashmap.insert(
        Command::ListPodcasts.to_string(),
        build_list_podcasts_command,
    );
    factory_hashmap.insert(
        Command::ListEpisodes.to_string(),
        build_list_episodes_command,
    );
    factory_hashmap.insert(
        Command::Select("".to_string()).to_string(),
        build_select_command,
    );
    factory_hashmap.insert(
        Command::AddRss(CommandUrl(Url::parse("https://www.google.com").unwrap())).to_string(),
        build_add_rss_command,
    );
    factory_hashmap.insert(
        Command::Advance(CommandDuration(chrono::Duration::seconds(0))).to_string(),
        build_advance_command,
    );
    factory_hashmap.insert(
        Command::GoBack(CommandDuration(chrono::Duration::seconds(0))).to_string(),
        build_go_back_command,
    );
    factory_hashmap.insert(Command::Help(None).to_string(), build_help_command);
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
