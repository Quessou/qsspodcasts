use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex, MutexGuard};

use log::info;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::mp3_player::Mp3Player;
use crate::player_error::PlayerError;

use path_providing::path_provider::PathProvider;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct RodioMp3Player {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    selected_episode: Option<PodcastEpisode>,
    path_provider: Arc<Mutex<Box<dyn PathProvider>>>,
}

impl RodioMp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> RodioMp3Player {
        let (stream, stream_handle): (OutputStream, OutputStreamHandle) =
            OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        RodioMp3Player {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            selected_episode: None,
            path_provider: Arc::new(Mutex::new(path_provider)),
        }
    }
}

impl Mp3Player for RodioMp3Player {
    fn get_path_provider(&self) -> MutexGuard<Box<dyn PathProvider>> {
        self.path_provider.lock().unwrap()
    }

    fn get_selected_episode(&self) -> &Option<PodcastEpisode> {
        &self.selected_episode
    }

    fn set_selected_episode(&mut self, episode: Option<PodcastEpisode>) {
        self.selected_episode = episode;
    }

    fn pause(&mut self) {
        self.sink.lock().unwrap().pause();
        info!("Player paused");
    }

    fn play(&mut self) {
        self.sink.lock().unwrap().play();
        info!("Player started");
    }
    fn is_paused(&self) -> bool {
        self.sink.lock().unwrap().is_paused()
    }

    /// Play the audio file whose path is given in parameter
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to be played
    ///
    /// # TODO
    /// * Replace calls to std::fs::File to tokio::fs::File
    fn play_file(&mut self, path: &str) -> Result<(), PlayerError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(PlayerError::from(e)),
        };
        let file = BufReader::new(file);

        let source = match Decoder::new_mp3(file) {
            Ok(s) => s,
            Err(e) => return Err(PlayerError::from(e)),
        };
        self.sink.lock().unwrap().append(source);
        info!("File {path} started");

        Ok(())
    }
}

// NOTE : Is this really the only solution ?
unsafe impl Send for RodioMp3Player {}
