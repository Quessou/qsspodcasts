use log::debug;
use podcast_management::data_objects::hashable::Hashable;
use podcast_management::data_objects::podcast::Podcast;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use url::Url;

use crate::command_error::{self, CommandError, ErrorKind as CommandErrorKind};
use crate::commands::command_enum::Command;
use crate::commands::helps::{
    command_help_library::CommandHelpLibrary,
    command_help_library_builder::get_command_help_library,
};
use crate::output::output_type::OutputType;

use business_core::business_core::BusinessCore;
use data_transport::{AutocompleterMessageType, DataSender};
use podcast_management::data_objects::podcast_episode::PodcastEpisode;
pub use podcast_management::podcast_library::PodcastLibrary;
pub use podcast_player::players::mp3_player::Mp3Player;

pub struct CommandExecutor {
    core: Arc<TokioMutex<BusinessCore>>,
    command_help_library: CommandHelpLibrary,
    autocompleter_command_sender: Option<DataSender<AutocompleterMessageType>>,
}

impl CommandExecutor {
    pub fn new(
        business_core: Arc<TokioMutex<BusinessCore>>,
        autocompleter_command_sender: Option<DataSender<AutocompleterMessageType>>,
    ) -> CommandExecutor {
        let command_help_library = get_command_help_library();
        CommandExecutor {
            core: business_core,
            command_help_library,
            autocompleter_command_sender,
        }
    }

    pub async fn initialize(&mut self) {
        let mut core = self.core.lock().await;
        core.initialize();
        core.build_podcasts().await;
    }

    async fn handle_play(&mut self, _: Command) -> Result<OutputType, CommandError> {
        match self.core.lock().await.play().await {
            Ok(_) => Ok(OutputType::None),
            Err(e) => Err(CommandError::new(
                Some(Box::new(e)),
                CommandErrorKind::ExecutionFailed,
                Some("play".to_owned()),
                None,
            )),
        }
    }

    async fn handle_pause(&mut self, _: Command) -> Result<OutputType, CommandError> {
        match self.core.lock().await.pause().await {
            Ok(_) => Ok(OutputType::None),
            Err(e) => Err(CommandError::new(
                Some(Box::new(e)),
                CommandErrorKind::ExecutionFailed,
                Some("pause".to_owned()),
                None,
            )),
        }
    }

    async fn update_autocompleter_hashes(&mut self, hashes: Vec<String>) -> Result<(), ()> {
        if let Some(sender) = self.autocompleter_command_sender.as_mut() {
            let res = sender
                .send(AutocompleterMessageType::HashUpdate(hashes))
                .await;
            return res;
        }
        debug!("No autocompleter message sender set");
        Ok(())
    }

    async fn handle_list_podcasts(&mut self, _: Command) -> Result<OutputType, CommandError> {
        let tmp_core = self.core.lock().await;
        let podcast_library = tmp_core.podcast_library.lock().await;
        let podcasts = &podcast_library.podcasts;

        let podcasts = podcasts
            .iter()
            .map(|p| p.shallow_copy())
            .collect::<Vec<Podcast>>();
        drop(podcast_library);
        drop(tmp_core);

        let hashes = podcasts.iter().map(|p| p.hash()).collect();
        self.update_autocompleter_hashes(hashes)
            .await
            .expect("Sending of new hashes to autocompleter failed");

        Ok(OutputType::Podcasts(podcasts))
    }

    pub async fn clean(&mut self) {
        self.core.lock().await.clean().await;
    }

    async fn handle_list_episodes(
        &mut self,
        _: Command,
        hash: Option<String>,
    ) -> Result<OutputType, CommandError> {
        let tmp_core = self.core.lock().await;
        let podcast_library = tmp_core.podcast_library.lock().await;
        let podcasts = &podcast_library.podcasts;

        let episodes_iter = podcasts
            .iter()
            .flat_map(|p| p.episodes.clone())
            .filter(|e| {
                if hash.is_none() {
                    return true;
                }

                let p = podcasts
                    .iter()
                    .find(|p| &p.hash() == hash.as_ref().unwrap());
                let title = if let Some(t) = &p { &t.title } else { "" };

                e.podcast_name == title
            });

        let mut episodes: Vec<PodcastEpisode> = episodes_iter.collect();
        episodes.sort_by(|p1, p2| p1.pub_date.cmp(&p2.pub_date).reverse());
        drop(podcast_library);
        drop(tmp_core);

        let hashes = episodes.iter().map(|p| p.hash()).collect();
        self.update_autocompleter_hashes(hashes)
            .await
            .expect("Sending of new hashes to autocompleter failed");

        Ok(OutputType::Episodes(episodes))
    }

    async fn search_episode(&self, hash: &str) -> Option<PodcastEpisode> {
        let tmp_core = self.core.lock().await;
        let episode = tmp_core.podcast_library.lock().await.search_episode(hash);
        drop(tmp_core);
        episode
    }

    async fn select_episode(&mut self, hash: &str) -> Result<OutputType, CommandError> {
        if let Some(ep) = self.search_episode(hash).await {
            if self.core.lock().await.download_episode(&ep).await.is_err() {
                return Err(CommandError::new(
                    None,
                    CommandErrorKind::DownloadFailed,
                    Some(format!("select {}", hash)),
                    Some("Episode download failed".to_string()),
                ));
            }
            if self.core.lock().await.select_episode(&ep).await.is_err() {
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
        if let Err(e) = self.core.lock().await.add_url(&url).await {
            return Err(CommandError::new(
                Some(Box::new(e)),
                command_error::ErrorKind::ExecutionFailed,
                None,
                Some("URL writing failed".to_string()),
            ));
        }
        if self.core.lock().await.load_feed(&url).await.is_err() {
            return Err(CommandError::new(
                None,
                command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Loading of new RSS feed failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn delete_rss(&mut self, hash: &str) -> Result<OutputType, CommandError> {
        if let Err(e) = self.core.lock().await.delete_rss(hash).await {
            return Err(CommandError::new(
                Some(Box::new(e)),
                command_error::ErrorKind::ExecutionFailed,
                None,
                Some("URL deletion failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn advance_in_podcast(
        &mut self,
        duration: chrono::Duration,
    ) -> Result<OutputType, CommandError> {
        if self.core.lock().await.seek(duration).await.is_err() {
            return Err(CommandError::new(
                None,
                command_error::ErrorKind::ExecutionFailed,
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
        if self.core.lock().await.seek(duration * -1).await.is_err() {
            return Err(CommandError::new(
                None,
                command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Seeking failed".to_string()),
            ));
        }
        Ok(OutputType::None)
    }

    async fn handle_mark_as_finished_command(&mut self) -> Result<OutputType, CommandError> {
        match self
            .core
            .lock()
            .await
            .mark_current_podcast_as_finished()
            .await
        {
            Ok(_) => Ok(OutputType::None),
            Err(_) => Err(CommandError::new(
                None,
                command_error::ErrorKind::ExecutionFailed,
                None,
                Some("Marking as finished failed".to_string()),
            )),
        }
    }
    async fn handle_latest_podcasts_command(&mut self) -> Result<OutputType, CommandError> {
        let tmp_core = self.core.lock().await;
        let library = tmp_core.podcast_library.lock().await;

        let all_episodes = library.podcasts.iter().flat_map(|p| &p.episodes);
        let latest_episodes = all_episodes
            .filter(|e| e.was_published_recently())
            .cloned()
            .collect();

        drop(library);
        drop(tmp_core);
        Ok(OutputType::Episodes(latest_episodes))
    }

    fn handle_help_command(&mut self, command: Option<String>) -> Result<OutputType, CommandError> {
        let helps = match command {
            Some(c) => match self.command_help_library.get_description(&c) {
                Some(h) => vec![h],
                None => vec![],
            },
            None => self.command_help_library.get_descriptions(),
        };

        if helps.is_empty() {
            return Err(CommandError::new(
                None,
                command_error::ErrorKind::ObjectNotFound,
                Some("help".to_string()),
                Some("Command name unknown".to_string()),
            ));
        }

        Ok(OutputType::CommandHelps(helps))
    }

    async fn handle_set_volume_command(
        &mut self,
        new_volume: u32,
    ) -> Result<OutputType, CommandError> {
        if let Err(e) = self.core.lock().await.set_volume(new_volume).await {
            return Err(CommandError::new(
                Some(Box::new(e)),
                CommandErrorKind::ExecutionFailed,
                Some("set_volume".to_owned()),
                Some("Volume changing failed".to_owned()),
            ));
        }
        Ok(OutputType::None)
    }
    async fn handle_volume_offset_command(
        &mut self,
        volume_offset: i32,
    ) -> Result<OutputType, CommandError> {
        if let Err(e) = self
            .core
            .lock()
            .await
            .add_volume_offset(volume_offset)
            .await
        {
            return Err(CommandError::new(
                Some(Box::new(e)),
                CommandErrorKind::ExecutionFailed,
                Some("volume_offset".to_owned()),
                Some("Volume changing failed".to_owned()),
            ));
        }
        Ok(OutputType::None)
    }

    pub async fn execute_command(&mut self, command: Command) -> Result<OutputType, CommandError> {
        let command_output = match command {
            Command::Pause => self.handle_pause(command).await?,
            Command::Play(Some(ref hash)) => {
                self.select_episode(hash).await?;
                self.handle_play(command).await?
            }
            Command::Play(None) => self.handle_play(command).await?,
            Command::Exit => OutputType::None,
            Command::Help(command) => self.handle_help_command(command)?,
            Command::ListPodcasts => self.handle_list_podcasts(command).await?,
            Command::ListEpisodes(ref hash) => {
                let hash = hash.clone();
                self.handle_list_episodes(command, hash).await?
            }
            Command::Select(hash) => self.select_episode(&hash).await?,
            Command::AddRss(url) => self.add_rss(&url.0).await?,
            Command::DeleteRss(hash) => self.delete_rss(&hash).await?,
            Command::Advance(duration) => self.advance_in_podcast(duration.0).await?,
            Command::GoBack(duration) => self.go_back_in_podcast(duration.0).await?,
            Command::MarkAsFinished => self.handle_mark_as_finished_command().await?,
            Command::LatestPodcasts => self.handle_latest_podcasts_command().await?,
            Command::VolumeUp(offset) => self.handle_volume_offset_command(offset as i32).await?,
            Command::VolumeDown(offset) => {
                self.handle_volume_offset_command(-(offset as i32)).await?
            }
            Command::SetVolume(new_volume) => self.handle_set_volume_command(new_volume).await?,
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
    use podcast_player::player_error::{ErrorKind, PlayerError};
    use podcast_player::players::mp3_player::Mp3Player as TraitMp3Player;

    use std::sync::Arc;
    use test_case::test_case;
    use tokio::sync::Mutex as TokioMutex;

    fn instanciate_mock_mp3_player() -> Arc<TokioMutex<MockMp3Player>> {
        Arc::new(TokioMutex::new(MockMp3Player::new()))
    }

    async fn instanciate_executor(
        mp3_player: Arc<TokioMutex<dyn TraitMp3Player + Send + Sync>>,
    ) -> CommandExecutor {
        let core =
            BusinessCore::new_in_arc(mp3_player, Arc::new(DummyPathProvider::new("")), None).await;
        CommandExecutor::new(core, None)
    }

    #[test]
    pub fn test_executor_instanciation() -> Result<(), String> {
        let mp3_player = instanciate_mock_mp3_player();
        let _executor = instanciate_executor(mp3_player);
        Ok(())
    }

    /// .
    ///
    ///
    /// # TODO  
    /// - Find a way to circumvent the issues related to mockall to make this test relevant
    /// again
    #[test_case(true, 0, 0 => Err(PlayerError::new(None, ErrorKind::AlreadyPaused)); "Returns an AlreadyPaused error if the player is already paused")]
    #[test_case(false, 0, 0 => Err(PlayerError::new(None, ErrorKind::AlreadyPaused)); "Returns also an error otherwise")]
    #[tokio::test]
    pub async fn test_execute_pause_command(
        player_paused: bool,
        is_paused_call_count: usize,
        pause_call_count: usize,
    ) -> Result<(), PlayerError> {
        let mp3_player = instanciate_mock_mp3_player();
        // Setting up expectations
        mp3_player
            .lock()
            .await
            .expect_is_paused()
            .times(is_paused_call_count)
            .return_const(player_paused);
        mp3_player
            .lock()
            .await
            .expect_pause()
            .times(pause_call_count)
            .return_const(());
        mp3_player
            .lock()
            .await
            .expect_get_selected_episode()
            .return_const(None);
        mp3_player
            .lock()
            .await
            .expect_register_observer()
            .return_const(());

        let mut executor = instanciate_executor(mp3_player).await;

        // TODO : There's something wrong in the mock expectations. Fix it.
        match executor.execute_command(Command::Pause).await {
            Ok(_) => Ok(()),
            Err(_) => Err(PlayerError::new(None, ErrorKind::AlreadyPaused)),
        }
    }

    /// .
    ///
    /// # TODO  
    /// - Find a way to circumvent the issues related to mockall to make this test relevant
    /// again
    #[allow(unused_must_use)]
    #[ignore = "Irrelevant test since I added a check on get_selected_episode()"]
    #[test_case(true, 1, 1 => Ok(()); "Launches the player if it is paused")]
    #[test_case(false, 1, 0 => Ok(()); "Does not launch the player if it is not paused")]
    #[tokio::test]
    pub async fn test_execute_play_command(
        player_paused: bool,
        is_paused_call_count: usize,
        play_call_count: usize,
    ) -> Result<(), String> {
        let mp3_player = instanciate_mock_mp3_player();
        // Setting up expectations
        mp3_player
            .lock()
            .await
            .expect_is_paused()
            .times(is_paused_call_count)
            .return_const(player_paused);
        mp3_player
            .lock()
            .await
            .expect_play()
            .times(play_call_count)
            .return_const(());

        let mut executor = instanciate_executor(mp3_player).await;

        executor.execute_command(Command::Play(None)).await;

        Ok(())
    }
}
