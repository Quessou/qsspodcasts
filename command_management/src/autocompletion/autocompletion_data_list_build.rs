use crate::autocompletion::autocompletion_command_data::AutocompletionCommandData;
use crate::commands::command_enum::Command;

use strum::IntoEnumIterator;

use super::command_parameter_type::CommandParameterType;
//use strum_macros::EnumIter;

pub fn command_to_parameters(c: &Command) -> Vec<CommandParameterType> {
    match c {
        Command::Help(_) => vec![CommandParameterType::CommandName],
        Command::AddRss(_) => vec![CommandParameterType::Url],
        Command::Select(_) => vec![CommandParameterType::Hash],
        Command::Advance(_) => vec![CommandParameterType::Duration],
        Command::GoBack(_) => vec![CommandParameterType::Duration],
        _ => vec![],
    }
}

pub fn build_command_autocompletion_data_list() -> Vec<AutocompletionCommandData> {
    Command::iter()
        .map(|c| {
            let parameters = command_to_parameters(&c);
            AutocompletionCommandData::new(c, parameters)
        })
        .collect()
}
