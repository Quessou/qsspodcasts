use crate::command_error::{CommandError, ErrorKind as CommandErrorKind};
use crate::commands::command_enum::Command;
use crate::output::output_type::OutputType;

pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::players::mp3_player::Mp3Player;

use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

pub struct CommandExecutor {
    podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
}

impl CommandExecutor {
    pub fn new(
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
    ) -> CommandExecutor {
        CommandExecutor {
            podcast_library,
            mp3_player,
        }
    }

    async fn handle_play(&self, _: Command) -> Result<OutputType, CommandError> {
        let mut mp3_player = self.mp3_player.lock().await;
        if mp3_player.is_paused() {
            mp3_player.play();
        }
        let return_message = String::from("Player launched");
        Ok(OutputType::RawString(return_message))
    }

    async fn handle_pause(&self, _: Command) -> Result<OutputType, CommandError> {
        let mut mp3_player = self.mp3_player.lock().await;
        if !mp3_player.is_paused() {
            mp3_player.pause();
        }
        let return_message = String::from("Player paused");
        Ok(OutputType::RawString(return_message))
    }

    async fn handle_list_podcasts(&self, _: Command) -> Result<OutputType, CommandError> {
        let podcast_library = self.podcast_library.lock().await;
        let podcasts = &podcast_library.podcasts;

        let podcasts = podcasts.iter().map(|p| p.shallow_copy()).collect();

        Ok(OutputType::Podcasts(podcasts))
    }
    /// Executes command
    pub async fn execute_command(&mut self, command: Command) -> Result<OutputType, CommandError> {
        let command_output = match command {
            Command::Pause => self.handle_pause(command).await?,
            Command::Play => self.handle_play(command).await?,
            Command::Exit => OutputType::RawString(String::from("Exiting")),
            Command::ListPodcasts => self.handle_list_podcasts(command).await?,
            _ => {
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::UnhandledCommand,
                    None,
                    Some(format!("Command {:#?} unhandled by executor", command)),
                ))
            }
        };

        Ok(command_output)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::mocks::mp3_player::MockMp3Player;

    use podcast_player::players::mp3_player::Mp3Player as TraitMp3Player;
    use test_case::test_case;

    use tokio_test;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    fn instanciate_mock_mp3_player() -> Arc<TokioMutex<MockMp3Player>> {
        Arc::new(TokioMutex::new(MockMp3Player::new()))
    }

    fn instanciate_executor(
        mp3_player: Arc<TokioMutex<dyn TraitMp3Player + Send>>,
    ) -> CommandExecutor {
        let podcast_library = PodcastLibrary::new();
        CommandExecutor::new(Arc::new(TokioMutex::new(podcast_library)), mp3_player)
    }

    #[test]
    pub fn test_executor_instanciation() -> Result<(), String> {
        let mp3_player = instanciate_mock_mp3_player();
        let _executor = instanciate_executor(mp3_player);
        Ok(())
    }

    #[test_case(true, 1, 0 => Ok(()); "Returns ok if the player is already paused")]
    #[test_case(false, 1, 1 => Ok(()); "Returns also ok otherwise")]
    pub fn test_execute_pause_command(
        player_paused: bool,
        is_paused_call_count: usize,
        pause_call_count: usize,
    ) -> Result<(), String> {
        let mp3_player = instanciate_mock_mp3_player();
        // Setting up expectations
        aw!(mp3_player.lock())
            .expect_is_paused()
            .times(is_paused_call_count)
            .return_const(player_paused);
        aw!(mp3_player.lock())
            .expect_pause()
            .times(pause_call_count)
            .return_const(());

        let mut executor = instanciate_executor(mp3_player);

        aw!(executor.execute_command(Command::Pause));

        Ok(())
    }

    #[test_case(true, 1, 1 => Ok(()); "Launches the player if it is paused")]
    #[test_case(false, 1, 0 => Ok(()); "Does not launch the player if it is not paused")]
    pub fn test_execute_play_command(
        player_paused: bool,
        is_paused_call_count: usize,
        play_call_count: usize,
    ) -> Result<(), String> {
        let mp3_player = instanciate_mock_mp3_player();
        // Setting up expectations
        aw!(mp3_player.lock())
            .expect_is_paused()
            .times(is_paused_call_count)
            .return_const(player_paused);
        aw!(mp3_player.lock())
            .expect_play()
            .times(play_call_count)
            .return_const(());

        let mut executor = instanciate_executor(mp3_player);

        aw!(executor.execute_command(Command::Play));

        Ok(())
    }
}
