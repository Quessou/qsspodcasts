use std::fs::File;
use std::io::BufReader;

use log::{error, info, warn};

use rodio::{Decoder, OutputStream, Sink};

use crate::player_error::PlayerError;

use path_providing::path_provider::PathProvider;
use podcast_management::data_objects::podcast_episode::PodcastEpisode;

pub struct Mp3Player {
    sink: Sink,
    _stream: OutputStream,
    selected_episode: Option<PodcastEpisode>,
    path_provider: Box<dyn PathProvider>,
}

impl Mp3Player {
    pub fn new(path_provider: Box<dyn PathProvider>) -> Mp3Player {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Mp3Player {
            sink,
            _stream: stream,
            selected_episode: None,
            path_provider,
        }
    }

    /// TODO: FIXME : https://rust-lang.github.io/rust-clippy/master/index.html#result_unit_err
    pub fn select_episode(&mut self, episode: &PodcastEpisode) -> Result<(), ()> {
        if !self.path_provider.compute_episode_path(episode).exists() {
            warn!("Cannot select an episode which has not been downloaded first");
            return Err(());
        }
        self.selected_episode = Some(episode.clone());
        Ok(())
    }

    pub fn play_selected_episode(&mut self) -> Result<(), PlayerError> {
        let path = self
            .path_provider
            .compute_episode_path(self.selected_episode.as_ref().unwrap())
            .into_os_string()
            .into_string()
            .unwrap();

        self.play_file(&path)
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

        let source = match Decoder::new(file) {
            Ok(s) => s,
            Err(e) => return Err(PlayerError::from(e)),
        };
        self.sink.append(source);

        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn play(&mut self) {
        self.sink.play();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
}
