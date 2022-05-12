use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, OutputStream, Sink};

use crate::player_error::PlayerError;

pub struct Mp3Player {
    sink: Sink,
    _stream: OutputStream,
}

impl Mp3Player {
    pub fn new() -> Mp3Player {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Mp3Player { sink, _stream: stream }
    }

    /// Play the audio file whose path is given in parameter
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to be played
    ///
    /// # TODO
    /// * Replace calls to std::fs::File to tokio::fs::File
    pub fn play_file(&mut self, path: &str) -> Result<(), PlayerError> {
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
}

impl Default for Mp3Player {
    fn default() -> Self {
        Self::new()
    }
}
