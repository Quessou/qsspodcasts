use business_core::notification::Notification;
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use crate::command_error::CommandError;
use crate::command_executor::CommandExecutor;
use crate::command_parser::CommandParser;
use crate::commands::command_enum::Command;
use crate::output::output_type::OutputType;
use data_transport::{DataReceiver, DataSender};

pub type CommandResult = Result<OutputType, CommandError>;

pub struct CommandEngine {
    command_parser: Arc<TokioMutex<CommandParser>>,
    command_executor: CommandExecutor,
    command_receiver: Option<DataReceiver<String>>,
    output_sender: Option<DataSender<CommandResult>>,
    notifications_sender: Option<DataSender<Notification>>,
}

impl CommandEngine {
    /// Creates a new [`CommandEngine`].
    pub fn new(
        command_executor: CommandExecutor,
        command_receiver: Option<DataReceiver<String>>,
        output_sender: Option<DataSender<CommandResult>>,
        notifications_sender: Option<DataSender<Notification>>,
    ) -> CommandEngine {
        CommandEngine {
            command_parser: Arc::new(TokioMutex::new(CommandParser::new())),
            command_executor,
            command_receiver,
            output_sender,
            notifications_sender,
        }
    }

    async fn clean(&mut self) {
        self.command_executor.clean().await;
    }

    pub async fn handle_command(&mut self, command: &str) -> Result<OutputType, CommandError> {
        let command = match self.command_parser.lock().await.parse_command(command) {
            Ok(c) => c,
            Err(e) => {
                self.notifications_sender
                    .as_mut()
                    .unwrap()
                    .send(Notification::Message(e.message.as_ref().unwrap().clone()))
                    .await
                    .unwrap();

                error!("Command parsing failed");
                return Err(e);
            }
        };

        if command == Command::Exit {
            self.notifications_sender
                .as_mut()
                .unwrap()
                .send(Notification::Message("Exiting...".to_owned()))
                .await
                .unwrap();
            self.clean().await;
            self.command_receiver.as_mut().unwrap().close();
        }

        self.command_executor.execute_command(command).await
    }

    pub async fn run(&mut self) {
        if self.command_receiver.is_none() {
            error!("Need a command receiver to use the run method");
            return;
        }
        self.command_executor.initialize().await;
        while let Some(command) = self.command_receiver.as_mut().unwrap().receive().await {
            let output = self.handle_command(&command).await;
            if self
                .output_sender
                .as_mut()
                .unwrap()
                .send(output)
                .await
                .is_err()
            {
                error!("Could not send output in channel");
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use business_core::business_core::BusinessCore;
    use path_providing::dummy_path_provider::DummyPathProvider;
    use podcast_player::players::mp3_player::Mp3Player;

    use super::*;
    use crate::mocks::mp3_player::MockMp3Player;
    use test_case::test_case;

    async fn instanciate_engine(
        player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
    ) -> CommandEngine {
        let core =
            BusinessCore::new_in_arc(player, Arc::new(DummyPathProvider::new("")), None).await;
        let executor = CommandExecutor::new(core, None);
        CommandEngine::new(executor, None, None, None)
    }

    #[test]
    fn test_engine_instanciation() -> Result<(), String> {
        let _engine = instanciate_engine(Arc::new(TokioMutex::new(MockMp3Player::new())));
        Ok(())
    }

    #[ignore = "Made deprecated by changes in sanity checks in play/pause methods of Mp3 players"]
    #[test_case(true, 1 => Ok(()))]
    #[test_case(false, 0 => Ok(()))]
    #[tokio::test]
    async fn test_play_command(is_paused: bool, expected_play_calls: usize) -> Result<(), String> {
        let mut player = MockMp3Player::new();

        player.expect_is_paused().times(1).return_const(is_paused);
        player
            .expect_play()
            .times(expected_play_calls)
            .return_const(());

        let mut engine = instanciate_engine(Arc::new(TokioMutex::new(player))).await;

        match engine.handle_command("play").await {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Something went wrong")),
        }
    }
}
