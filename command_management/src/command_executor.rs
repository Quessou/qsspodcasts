use crate::commands::command_enum::Command;
pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::mp3_player::Mp3Player;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type CommandExecutionFn = fn(&CommandExecutor, Command) -> Result<String, ()>;

pub struct CommandExecutor {
    podcast_library: Arc<Mutex<PodcastLibrary>>,
    mp3_player: Arc<Mutex<Mp3Player>>,
    command_callbacks: HashMap<Command, CommandExecutionFn>,
}

impl CommandExecutor {
    pub fn new(
        podcast_library: Arc<Mutex<PodcastLibrary>>,
        mp3_player: Arc<Mutex<Mp3Player>>,
    ) -> CommandExecutor {
        // Note : Why does it seem to not work with HashMap::from ?
        let mut command_callbacks: HashMap<Command, CommandExecutionFn> = HashMap::new();
        command_callbacks.insert(Command::Play, CommandExecutor::handle_play);
        command_callbacks.insert(Command::Pause, CommandExecutor::handle_pause);

        CommandExecutor {
            podcast_library,
            mp3_player,
            command_callbacks,
        }
    }

    fn handle_play(&self, _: Command) -> Result<String, ()> {
        let mut mp3_player = self.mp3_player.lock().unwrap();
        if mp3_player.is_paused() {
            mp3_player.play();
        }
        Ok("Player launched".to_string())
    }

    fn handle_pause(&self, _: Command) -> Result<String, ()> {
        let mut mp3_player = self.mp3_player.lock().unwrap();
        if !mp3_player.is_paused() {
            mp3_player.pause();
        }
        Ok("Player paused".to_string())
    }

    /// Executes command
    ///
    /// # TODO :
    /// * Add error type for command execution
    pub fn execute_command(&mut self, command: Command) -> Result<String, ()> {
        let handler = self.command_callbacks.get(&command);
        let return_message: String = match handler {
            Some(f) => f(self, command).unwrap(),
            None => return Err(()),
        };
        Ok(return_message)
    }
}
