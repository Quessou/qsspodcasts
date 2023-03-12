use super::command_parameter_type::CommandParameterType;
use crate::commands::command_enum::Command;

pub struct AutocompletionCommandData {
    command: Command,
    parameters_types: Vec<CommandParameterType>,
}

impl AutocompletionCommandData {
    pub fn new(
        command: Command,
        parameters_types: Vec<CommandParameterType>,
    ) -> AutocompletionCommandData {
        AutocompletionCommandData {
            command,
            parameters_types,
        }
    }
}
