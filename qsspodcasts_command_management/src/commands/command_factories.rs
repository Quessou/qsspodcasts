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

pub fn build_list_episodes_command(mut parameters: Vec<String>) -> Result<Command, CommandError> {
    let command = if !parameters.is_empty() && is_hash(&parameters[0]) {
        let hash = parameters.pop().unwrap();
        Command::ListEpisodes(Some(hash))
    } else if !parameters.is_empty() && !is_hash(&parameters[0]) {
        let error = build_parsing_failed_error("Play", "Parsing of hash failed");
        return Err(error);
    } else {
        Command::ListEpisodes(None)
    };
    Ok(command)
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
        return Err(build_parsing_failed_error(
            "add_url",
            &format!("Parsing of URL {} parsing failed", &parameters[0]),
        ));
    }

    Ok(Command::AddRss(CommandUrl(url.unwrap())))
}

pub fn build_delete_rss_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("delete_rss"));
    }

    let hash = &parameters[0];
    if hash.len() != HASH_LEN || !is_hash(hash) {
        return Err(build_parsing_failed_error(
            "delete_rss",
            "Parameter parsing failed",
        ));
    }
    Ok(Command::DeleteRss(hash.to_string()))
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

pub fn build_mark_as_finished_command(_parameters: Vec<String>) -> Result<Command, CommandError> {
    Ok(Command::MarkAsFinished)
}

pub fn build_get_latest_podcasts_command(
    _parameters: Vec<String>,
) -> Result<Command, CommandError> {
    Ok(Command::LatestPodcasts)
}

pub fn build_volume_up_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("volume_up"));
    }
    let volume_offset = parameters[0].parse::<u32>();
    if volume_offset.is_err() {
        return Err(build_parsing_failed_error("volume_up", "Not an integer"));
    }
    Ok(Command::VolumeUp(volume_offset.unwrap()))
}
pub fn build_volume_down_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("volume_down"));
    }
    let volume_offset = parameters[0].parse::<u32>();
    if volume_offset.is_err() {
        return Err(build_parsing_failed_error("volume_up", "Not an integer"));
    }
    Ok(Command::VolumeDown(volume_offset.unwrap()))
}
pub fn build_set_volume_command(parameters: Vec<String>) -> Result<Command, CommandError> {
    if parameters.len() != 1 {
        return Err(build_bad_parameter_count_error("set_volume"));
    }
    let volume_offset = parameters[0].parse::<u32>();
    if volume_offset.is_err() {
        return Err(build_parsing_failed_error("volume_up", "Not an integer"));
    }
    Ok(Command::SetVolume(volume_offset.unwrap()))
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
        Command::ListEpisodes(None).to_string(),
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
        Command::DeleteRss("".to_string()).to_string(),
        build_delete_rss_command,
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
    factory_hashmap.insert(
        Command::MarkAsFinished.to_string(),
        build_mark_as_finished_command,
    );
    factory_hashmap.insert(
        Command::LatestPodcasts.to_string(),
        build_get_latest_podcasts_command,
    );
    factory_hashmap.insert(Command::VolumeUp(0).to_string(), build_volume_up_command);
    factory_hashmap.insert(
        Command::VolumeDown(0).to_string(),
        build_volume_down_command,
    );
    factory_hashmap.insert(Command::SetVolume(0).to_string(), build_set_volume_command);
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
