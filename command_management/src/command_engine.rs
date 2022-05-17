use std::sync::{Arc, Mutex};

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::mp3_player::Mp3Player;

use crate::command_reader::read_command;

pub struct CommandEngine {
    mp3_player: Arc<Mutex<Mp3Player>>,
    podcast_library: Arc<Mutex<PodcastLibrary>>,
}

impl CommandEngine {
    pub fn new(
        mp3_player: Arc<Mutex<Mp3Player>>,
        podcast_library: Arc<Mutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine {
            mp3_player,
            podcast_library,
        }
    }

    pub fn handle_command(&mut self, command: &str) {
        let mut mp3_player = self.mp3_player.lock().unwrap();
        if mp3_player.is_paused() {
            mp3_player.play();
        } else {
            mp3_player.pause();
        }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let mut command = match read_command().await {
            Ok(c) => c,
            Err(_) => return Err(()),
        };
        let exit_command: String = String::from("exit");
        while exit_command != command {
            self.handle_command(&command);
            command = match read_command().await {
                Ok(c) => c,
                Err(_) => return Err(()),
            };
        }
        Ok(())
    }
}
