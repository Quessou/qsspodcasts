use crate::commands::command_enum::Command;
use std::collections::HashMap;
use std::vec::Vec;

use crate::commands::command_factories::{get_factory_hashmap, FactoryFn};

#[derive(Default)]
pub struct CommandParser {
    factory_hashmap: HashMap<String, FactoryFn>,
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
    /// Create an error type for parsing
    pub fn parse_command(&self, command: &str) -> Result<Command, ()> {
        let mut command_components = command.split(" ");
        let verb: String = command_components.next().unwrap().to_string();
        let parameters: Vec<String> = command_components.map(|s| s.to_string()).collect();
        if parameters.len() > 0 {
            println!("There are parameters to parse !")
        }
        let command = match self.factory_hashmap.get(&verb) {
            Some(factory) => factory(),
            None => return Err(()),
        };
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    /// TODO : Remember the way to mutualize tests
    use super::CommandParser;
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
}
