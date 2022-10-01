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

#[cfg(test)]
mod tests {

    use tokio_test;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    use super::*;
    use crate::mocks::mp3_player::MockMp3Player;
    use test_case::test_case;

    fn instanciate_engine(
        player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        library: Arc<TokioMutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine::new(player, library)
    }

    #[test]
    fn test_engine_instanciation() -> Result<(), String> {
        let player = MockMp3Player::new();
        let library = PodcastLibrary::new();
        let engine = instanciate_engine(
            Arc::new(TokioMutex::new(player)),
            Arc::new(TokioMutex::new(library)),
        );
        Ok(())
    }

    #[test_case(true, 1 => Ok(()))]
    #[test_case(false, 0 => Ok(()))]
    fn test_play_command(is_paused: bool, expected_play_calls: usize) -> Result<(), String> {
        let mut player = MockMp3Player::new();

        player.expect_is_paused().times(1).return_const(is_paused);
        player
            .expect_play()
            .times(expected_play_calls)
            .return_const(());

        let library = PodcastLibrary::new();
        let mut engine = instanciate_engine(
            Arc::new(TokioMutex::new(player)),
            Arc::new(TokioMutex::new(library)),
        );

        match aw!(engine.handle_command("play")) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(String::from("Something went wrong")),
        };
    }
}
