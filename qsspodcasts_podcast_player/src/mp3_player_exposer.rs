use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use crate::duration_wrapper::DurationWrapper;
use crate::enums::player_state::Mp3PlayerState;
use crate::players::mp3_player::Mp3Player;

/// Class that wraps an object implementing the Mp3Player trait and exposes only methods that allow
/// to retrieve data that are displayable on a user interface
///
/// TODO :
/// * Check if there isn't a more elegant solution
pub struct Mp3PlayerExposer {
    mp3_player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
}

impl Mp3PlayerExposer {
    pub fn new(mp3_player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>) -> Mp3PlayerExposer {
        Mp3PlayerExposer { mp3_player }
    }

    pub async fn get_selected_episode_duration(&self) -> Option<DurationWrapper> {
        self.mp3_player
            .lock()
            .await
            .get_selected_episode_duration()
            .await
    }

    pub async fn get_selected_episode_progression(&self) -> Option<DurationWrapper> {
        self.mp3_player
            .lock()
            .await
            .get_selected_episode_progression()
            .await
    }

    pub async fn get_selected_episode_progression_percentage(&self) -> Option<u8> {
        self.mp3_player
            .lock()
            .await
            .get_selected_episode_progression_percentage()
            .await
    }

    pub async fn is_paused(&self) -> bool {
        self.mp3_player.lock().await.is_paused()
    }
    pub async fn get_state(&self) -> Mp3PlayerState {
        self.mp3_player.lock().await.get_state()
    }
    pub async fn get_volume(&self) -> u32 {
        self.mp3_player.lock().await.get_volume()
    }
}
