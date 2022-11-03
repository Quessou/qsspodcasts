use log::error;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex as TokioMutex;

use crate::command_error::CommandError;
use crate::command_executor::CommandExecutor;
use crate::command_parser::CommandParser;
use crate::commands::command_enum::Command;
use crate::output::output_type::OutputType;

pub type CommandResult = Result<OutputType, CommandError>;

pub struct CommandEngine {
    command_parser: Arc<TokioMutex<CommandParser>>,
    command_executor: CommandExecutor,
    command_receiver: Receiver<String>,
    output_sender: Sender<CommandResult>,
}

impl CommandEngine {
    pub fn new(
        command_executor: CommandExecutor,
        command_receiver: Receiver<String>,
        output_sender: Sender<CommandResult>,
    ) -> CommandEngine {
        CommandEngine {
            command_parser: Arc::new(TokioMutex::new(CommandParser::new())),
            command_executor,
            command_receiver,
            output_sender,
        }
    }

    pub async fn handle_command(&mut self, command: &str) -> Result<OutputType, CommandError> {
        let command = match self.command_parser.lock().await.parse_command(command) {
            Ok(c) => c,
            Err(e) => {
                error!("Command parsing failed");
                return Err(e);
            }
        };

        if command == Command::Exit {
            self.command_receiver.close();
        }

        self.command_executor.execute_command(command).await
    }

    pub async fn run(&mut self) {
        self.command_executor.initialize().await;
        while let Some(command) = self.command_receiver.recv().await {
            let output = self.handle_command(&command).await;
            if self.output_sender.send(output).await.is_err() {
                error!("Could not send output in channel");
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use path_providing::dummy_path_provider::DummyPathProvider;
    use podcast_player::players::mp3_player::Mp3Player;
    use tokio_test;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    use super::*;
    use crate::mocks::mp3_player::MockMp3Player;
    use test_case::test_case;

    fn instanciate_engine(player: Arc<TokioMutex<dyn Mp3Player + Send>>) -> CommandEngine<'static> {
        CommandEngine::new(BusinessCore::new(
            player,
            Rc::new(DummyPathProvider::new("")),
        ))
    }

    #[test]
    fn test_engine_instanciation() -> Result<(), String> {
        let _engine = instanciate_engine(Arc::new(TokioMutex::new(MockMp3Player::new())));
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

        let mut engine = instanciate_engine(Arc::new(TokioMutex::new(player)));

        match aw!(engine.handle_command("play")) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(String::from("Something went wrong")),
        };
    }
}
