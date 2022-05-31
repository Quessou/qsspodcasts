use crate::commands::command_enum::Command;
pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::mp3_player::Mp3Player;

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

pub struct CommandExecutor {
    podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    mp3_player: Arc<TokioMutex<Mp3Player>>,
}

impl CommandExecutor {
    pub fn new(
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
        mp3_player: Arc<TokioMutex<Mp3Player>>,
    ) -> CommandExecutor {
        CommandExecutor {
            podcast_library,
            mp3_player,
        }
    }

    async fn handle_play(&self, _: Command) -> Result<String, ()> {
        let mut mp3_player = self.mp3_player.lock().await;
        if mp3_player.is_paused() {
            mp3_player.play();
            println!("toto");
        }
        Ok("Player launched".to_string())
    }

    async fn handle_pause(&self, _: Command) -> Result<String, ()> {
        let mut mp3_player = self.mp3_player.lock().await;
        if !mp3_player.is_paused() {
            mp3_player.pause();
        }
        Ok("Player paused".to_string())
    }

    /// Executes command
    ///
    /// # TODO :
    /// * Add error type for command execution
    pub async fn execute_command(&mut self, command: Command) -> Result<String, ()> {
        let return_message: String = match command {
            Command::Pause => self.handle_pause(command).await?,
            Command::Play => self.handle_play(command).await?,
            _ => return Err(()),
        };
        Ok(return_message)
    }
}