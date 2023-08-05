use crate::autocompletion::autocompletion_command_data::AutocompletionCommandData;
use crate::commands::command_enum::Command;

use strum::IntoEnumIterator;

use super::command_parameter_type::CommandParameterType;
//use strum_macros::EnumIter;

pub fn command_to_parameter(c: &Command) -> Option<CommandParameterType> {
    match c {
        Command::Help(_) => Some(CommandParameterType::CommandName),
        Command::AddRss(_) => Some(CommandParameterType::Url),
        Command::Select(_) => Some(CommandParameterType::Hash),
        Command::Play(_) => Some(CommandParameterType::Hash),
        Command::ListEpisodes(_) => Some(CommandParameterType::Hash),
        Command::Advance(_) => Some(CommandParameterType::Duration),
        Command::GoBack(_) => Some(CommandParameterType::Duration),
        _ => None,
    }
}

pub fn build_command_autocompletion_data_list() -> Vec<AutocompletionCommandData> {
    Command::iter()
        .map(|c| {
            let parameter = command_to_parameter(&c);
            AutocompletionCommandData::new(c, parameter)
        })
        .collect()
}
