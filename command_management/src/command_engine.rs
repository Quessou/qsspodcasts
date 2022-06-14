use log::error;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::players::mp3_player::Mp3Player;

use crate::command_error::CommandError;
use crate::command_executor::CommandExecutor;
use crate::command_parser::CommandParser;

pub struct CommandEngine {
    command_parser: Arc<TokioMutex<CommandParser>>,
    command_executor: CommandExecutor,
}

impl CommandEngine {
    pub fn new(
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine {
            command_parser: Arc::new(TokioMutex::new(CommandParser::new())),
            command_executor: CommandExecutor::new(podcast_library, mp3_player),
        }
    }

    pub async fn handle_command(&mut self, command: &str) -> Result<String, CommandError> {
        let command = match self.command_parser.lock().await.parse_command(command) {
            Ok(c) => c,
            Err(e) => {
                error!("Command parsing failed");
                return Err(e);
            }
        };

        let message = self
            .command_executor
            .execute_command(command)
            .await
            .unwrap();
        Ok(message)
    }
}
