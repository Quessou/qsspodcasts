use abstract_frontend::qss_podcast_frontend::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use abstract_frontend::QssPodcastFrontend;
use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::mp3_player::Mp3Player;

use crate::command_engine::CommandEngine;
use crate::command_read_utils::read_command;
use crate::prompt::{
    minimalistic_prompt_generator::MinimalisticPromptGenerator, prompt_writer::PromptWriter,
};

/// TODO : What to do with this ?
const EXIT_COMMAND: &str = "exit";

pub struct CommandFrontend {
    prompt_writer: Arc<TokioMutex<PromptWriter<MinimalisticPromptGenerator>>>,
    command_engine: Arc<TokioMutex<CommandEngine>>,
}

impl CommandFrontend {
    pub fn new(
        mp3_player: Arc<TokioMutex<Mp3Player>>,
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    ) -> CommandFrontend {
        CommandFrontend {
            prompt_writer: Arc::new(TokioMutex::new(PromptWriter::new(Box::new(
                MinimalisticPromptGenerator::new(),
            )))),
            command_engine: Arc::new(TokioMutex::new(CommandEngine::new(
                mp3_player,
                podcast_library,
            ))),
        }
    }

    pub async fn wait_for_command(&mut self) -> Result<String, ()> {
        self.prompt_writer.lock().await.write_prompt().await;
        match read_command().await {
            Ok(c) => Ok(c),
            Err(_) => Err(()),
        }
    }
}

#[async_trait]
impl QssPodcastFrontend for CommandFrontend {
    async fn run(&mut self) -> Result<(), ()> {
        let mut command = String::from("");
        while EXIT_COMMAND != command {
            command = match self.wait_for_command().await {
                Ok(c) => c,
                Err(_) => return Err(()),
            };
            self.command_engine
                .lock()
                .await
                .handle_command(&command)
                .await
                .expect("Command failed");
        }
        Ok(())
    }
}
