use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use crate::duration_wrapper::DurationWrapper;
use crate::players::mp3_player::{self, Mp3Player};

/// Class that wraps an object implementing the Mp3Player trait and exposes only methods that allow
/// to retrieve data that are displayable on a user interface
///
/// TODO :
/// * Check if there isn't a more elegant solution
pub struct Mp3PlayerExposer {
    mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
}

impl Mp3PlayerExposer {
    pub fn new(mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>) -> Mp3PlayerExposer {
        Mp3PlayerExposer { mp3_player }
    }

    pub async fn get_selected_episode_duration(&self) -> Option<DurationWrapper> {
        self.mp3_player.lock().await.get_selected_episode_duration()
    }

    pub async fn get_selected_episode_progression(&self) -> Option<DurationWrapper> {
        self.mp3_player
            .lock()
            .await
            .get_selected_episode_progression()
    }

    pub async fn get_selected_episode_progression_percentage(&self) -> Option<u8> {
        self.mp3_player
            .lock()
            .await
            .get_selected_episode_progression_percentage()
    }
}
