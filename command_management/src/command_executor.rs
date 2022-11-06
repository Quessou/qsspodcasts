use crate::command_error::{CommandError, ErrorKind as CommandErrorKind};
use crate::commands::command_enum::Command;
use crate::output::output_type::OutputType;

use business_core::business_core::BusinessCore;

use podcast_management::data_objects::podcast_episode::PodcastEpisode;
pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::players::mp3_player::Mp3Player;
use url::Url;

pub struct CommandExecutor {
    core: BusinessCore,
}

impl CommandExecutor {
    pub fn new(business_core: BusinessCore) -> CommandExecutor {
        CommandExecutor {
            core: business_core,
        }
    }

    pub async fn initialize(&mut self) {
        self.core.initialize();
        self.core.build_podcasts().await;
    }

    async fn handle_play(&mut self, _: Command) -> Result<OutputType, CommandError> {
        self.core.play().await;
        Ok(OutputType::None)
    }

    async fn handle_pause(&mut self, _: Command) -> Result<OutputType, CommandError> {
        self.core.pause().await;
        Ok(OutputType::None)
    }

    async fn handle_list_podcasts(&self, _: Command) -> Result<OutputType, CommandError> {
        let podcast_library = self.core.podcast_library.lock().await;
        let podcasts = &podcast_library.podcasts;

        let podcasts = podcasts.iter().map(|p| p.shallow_copy()).collect();

        Ok(OutputType::Podcasts(podcasts))
    }

    async fn handle_list_episodes(&self, _: Command) -> Result<OutputType, CommandError> {
        let podcast_library = self.core.podcast_library.lock().await;
        let podcasts = &podcast_library.podcasts;

        let mut episodes: Vec<PodcastEpisode> =
            podcasts.iter().flat_map(|p| p.episodes.clone()).collect();
        episodes.sort_by(|p1, p2| p1.pub_date.cmp(&p2.pub_date).reverse());

        Ok(OutputType::Episodes(episodes))
    }

    async fn search_episode(&self, hash: &str) -> Option<PodcastEpisode> {
        self.core.podcast_library.lock().await.search_episode(hash)
    }

    async fn select_episode(&mut self, hash: &str) -> Result<OutputType, CommandError> {
        if let Some(ep) = self.search_episode(hash).await {
            if self.core.download_episode(&ep).await.is_err() {
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::DownloadFailed,
                    Some(format!("select {}", hash)),
                    Some("Episode download failed".to_string()),
                ));
            }
            if self.core.select_episode(&ep).await.is_err() {
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::SelectionFailed,
                    Some(format!("select {}", hash)),
                    Some("Episode selection failed".to_string()),
                ));
            }
        } else {
            return Err(CommandError::new(
                None,
                CommandErrorKind::ObjectNotFound,
                Some(format!("select {}", hash)),
                Some("Episode not found".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn add_rss(&mut self, url: &Url) -> Result<OutputType, CommandError> {
        let url = url.to_string();
        if let Err(e) = self.core.add_url(&url).await {
            return Err(CommandError::new(
                Some(Box::new(e)),
                crate::command_error::ErrorKind::ExecutionFailed,
                None,
                Some("URL writing failed".to_string()),
            ));
        }
        if self.core.load_feed(&url).await.is_err() {
            return Err(CommandError::new(
                None,
                crate::command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Loading of new RSS feed failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn advance_in_podcast(
        &mut self,
        duration: chrono::Duration,
    ) -> Result<OutputType, CommandError> {
        if self.core.seek(duration).await.is_err() {
            return Err(CommandError::new(
                None,
                crate::command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Seeking failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn go_back_in_podcast(
        &mut self,
        duration: chrono::Duration,
    ) -> Result<OutputType, CommandError> {
        if self.core.seek(duration * -1).await.is_err() {
            return Err(CommandError::new(
                None,
                crate::command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Seeking failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    pub async fn execute_command(&mut self, command: Command) -> Result<OutputType, CommandError> {
        let command_output = match command {
            Command::Pause => self.handle_pause(command).await?,
            Command::Play => self.handle_play(command).await?,
            Command::Exit => OutputType::RawString(String::from("Exiting")),
            Command::ListPodcasts => self.handle_list_podcasts(command).await?,
            Command::ListEpisodes => self.handle_list_episodes(command).await?,
            Command::Select(hash) => self.select_episode(&hash).await?,
            Command::AddRss(url) => self.add_rss(&url).await?,
            Command::Advance(duration) => self.advance_in_podcast(duration).await?,
            Command::GoBack(duration) => self.go_back_in_podcast(duration).await?,
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

    use path_providing::dummy_path_provider::DummyPathProvider;
    use podcast_player::players::mp3_player::Mp3Player as TraitMp3Player;
    use std::rc::Rc;
    use std::sync::Arc;
    use test_case::test_case;
    use tokio::sync::Mutex as TokioMutex;

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
        let core = BusinessCore::new(mp3_player, Rc::new(DummyPathProvider::new("")), None);
        CommandExecutor::new(core)
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
