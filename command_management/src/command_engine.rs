use std::sync::{Arc, Mutex};

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::mp3_player::Mp3Player;

use crate::command_reader::read_command;
use crate::prompt::{
    minimalistic_prompt_generator::MinimalisticPromptGenerator, prompt_writer::PromptWriter,
};

const EXIT_COMMAND: &str = "exit";

pub struct CommandEngine {
    mp3_player: Arc<Mutex<Mp3Player>>,
    podcast_library: Arc<Mutex<PodcastLibrary>>,
    prompt_writer: PromptWriter,
}

impl CommandEngine {
    pub fn new(
        mp3_player: Arc<Mutex<Mp3Player>>,
        podcast_library: Arc<Mutex<PodcastLibrary>>,
    ) -> CommandEngine {
        CommandEngine {
            mp3_player,
            podcast_library,
            prompt_writer: PromptWriter::new(Box::new(MinimalisticPromptGenerator::new())),
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
