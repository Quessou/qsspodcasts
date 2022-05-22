use std::sync::{Arc, Mutex};

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::mp3_player::Mp3Player;

use crate::command_executor::CommandExecutor;
use crate::command_parser::CommandParser;
use crate::command_reader::read_command;
use crate::prompt::{
    minimalistic_prompt_generator::MinimalisticPromptGenerator, prompt_writer::PromptWriter,
};

const EXIT_COMMAND: &str = "exit";

pub struct CommandEngine {
    prompt_writer: PromptWriter,
    command_parser: CommandParser,
    command_executor: CommandExecutor,
}

impl CommandEngine {
    pub fn new(
        mp3_player: Arc<Mutex<Mp3Player>>,
        podcast_library: Arc<Mutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine {
            prompt_writer: PromptWriter::new(Box::new(MinimalisticPromptGenerator::new())),
            command_parser: CommandParser::new(),
            command_executor: CommandExecutor::new(podcast_library, mp3_player),
        }
    }

    pub fn handle_command(&mut self, command: &str) -> Result<(), ()> {
        let command = match self.command_parser.parse_command(command) {
            Ok(c) => c,
            Err(_) => return Err(()),
        };
        let message = self.command_executor.execute_command(command).unwrap();
        println!("{}", message);
        Ok(())
    }

    async fn wait_for_command(&mut self) -> Result<String, ()> {
        self.prompt_writer.write_prompt().await;
        match read_command().await {
            Ok(c) => Ok(c),
            Err(_) => Err(()),
        }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let mut command = String::from("");
        while EXIT_COMMAND != command {
            command = match self.wait_for_command().await {
                Ok(c) => c,
                Err(_) => return Err(()),
            };
            self.handle_command(&command);
        }
        Ok(())
    }
}
