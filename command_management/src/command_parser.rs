use log::{error, info};

use crate::command_error::{CommandError, ErrorKind as CommandErrorKind};
use crate::commands::command_enum::Command;
use std::collections::HashMap;
use std::vec::Vec;

use crate::commands::command_factories::{get_factory_hashmap, FactoryFn};

#[derive(Default)]
pub struct CommandParser {
    factory_hashmap: HashMap<&'static str, FactoryFn>,
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {
            factory_hashmap: get_factory_hashmap(),
        }
    }

    /// Parses command and returns a Result with a nested command object in it if parsing succeded
    ///
    /// # TODO
    /// * Create an error type for parsing
    /// * Add management of parameters
    pub fn parse_command(&self, command: &str) -> Result<Command, CommandError> {
        let mut command_components = command.split(' ');
        let verb = command_components.next().unwrap();
        let parameters: Vec<String> = command_components.map(|s| s.to_string()).collect();

        if !parameters.is_empty() {
            // TODO
            info!("There are parameters to parse !")
        }

        let command = match self.factory_hashmap.get(&verb.to_lowercase() as &str) {
            Some(factory) => match factory(parameters) {
                Ok(c) => c,
                Err(e) => return Err(e),
            },
            None => {
                let error_message = format!("Unknown verb {verb}");
                error!("{error_message}");
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::UnknownVerb,
                    Some(command.to_string()),
                    Some(error_message),
                ));
            }
        };
        Ok(command)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::commands::command_enum::Command;

    #[test]
    fn test_play() -> Result<(), String> {
        let command_parser = CommandParser::new();
        let command = command_parser.parse_command("play");
        assert_eq!(command.unwrap(), Command::Play);
        Ok(())
    }

    #[test]
    fn test_pause() -> Result<(), String> {
        let command_parser = CommandParser::new();
        let command = command_parser.parse_command("pause");
        assert_eq!(command.unwrap(), Command::Pause);
        Ok(())
    }

    #[test]
    fn test_invalid_verb() -> Result<(), String> {
        let command_parser = CommandParser::new();
        let error = command_parser.parse_command("toto").err().unwrap();
        assert_eq!(error.kind(), CommandErrorKind::UnknownVerb);
        Ok(())
    }
}
