use crate::command_error::{CommandError, ErrorKind as CommandErrorKind};
use crate::commands::command_enum::Command;

pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::{mp3_player::Mp3Player, rodio_mp3_player::RodioMp3Player};

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep as tokio_sleep;

pub struct CommandExecutor {
    podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    mp3_player: Arc<TokioMutex<RodioMp3Player>>,
}

impl CommandExecutor {
    pub fn new(
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
        mp3_player: Arc<TokioMutex<RodioMp3Player>>,
    ) -> CommandExecutor {
        CommandExecutor {
            podcast_library,
            mp3_player,
        }
    }

    async fn handle_play(&self, _: Command) -> Result<String, CommandError> {
        let mut mp3_player = self.mp3_player.lock().await;
        if mp3_player.is_paused() {
            mp3_player.play();
        }
        Ok("Player launched".to_string())
    }

    async fn handle_pause(&self, _: Command) -> Result<String, CommandError> {
        let mut mp3_player = self.mp3_player.lock().await;
        if !mp3_player.is_paused() {
            mp3_player.pause();
        }
        Ok("Player paused".to_string())
    }

    /// Executes command
    pub async fn execute_command(&mut self, command: Command) -> Result<String, CommandError> {
        let return_message: String = match command {
            Command::Pause => self.handle_pause(command).await?,
            Command::Play => self.handle_play(command).await?,
            Command::Exit => return Ok(String::from("Exiting")),
            _ => {
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::UnhandledCommand,
                    None,
                    Some(format!("Command {:#?} unhandled by executor", command)),
                ))
            }
        };
        Ok(return_message)
    }
}
