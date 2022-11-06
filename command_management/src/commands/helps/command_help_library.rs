use super::command_help::CommandHelp;
use super::command_help_register::CommandHelpRegister;
use std::collections::HashMap;

pub type CommandHelpMap = HashMap<&'static str, CommandHelpRegister>;

pub struct CommandHelpLibrary {
    descriptions: CommandHelpMap,
}

impl CommandHelpLibrary {
    pub fn new(descriptions: CommandHelpMap) -> CommandHelpLibrary {
        CommandHelpLibrary { descriptions }
    }

    /// TODO : Try to find a way to avoid allocations when calling get_short_help()
    pub fn get_descriptions(&self) -> Vec<CommandHelp> {
        self.descriptions
            .iter()
            .map(|(_, v)| v.get_short_help())
            .collect()
    }

    pub fn get_description(&self, command: &str) -> Option<CommandHelp> {
        let help = self.descriptions.get(command);
        match help {
            Some(h) => Some(h.get_detailed_help()),
            None => None,
        }
    }
}
