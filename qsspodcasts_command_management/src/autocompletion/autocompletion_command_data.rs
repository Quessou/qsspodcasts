use super::command_parameter_type::CommandParameterType;
use crate::commands::command_enum::Command;

pub struct AutocompletionCommandData {
    pub command: Command,
    pub parameter_type: Option<CommandParameterType>,
}

impl AutocompletionCommandData {
    pub fn new(
        command: Command,
        parameter_type: Option<CommandParameterType>,
    ) -> AutocompletionCommandData {
        AutocompletionCommandData {
            command,
            parameter_type,
        }
    }
}
