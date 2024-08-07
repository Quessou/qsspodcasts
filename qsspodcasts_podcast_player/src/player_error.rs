use std::error::Error;
use std::fmt;

use rodio::decoder::DecoderError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    Other,
    FileNotFound,
    RodioError,
    GStreamerError,
    NoEpisodeSelected,
    EpisodeAlreadySelected,
    AlreadyPlaying,
    AlreadyPaused,
}

/// Error type that wraps error that can come from the Player.
///
/// ### TODO
///
/// * Find  way to abstract the wrapping logic to reuse it somewhere else.
#[derive(Debug)]
pub struct PlayerError {
    source: Option<Box<dyn Error>>,
    kind: ErrorKind,
}

impl PlayerError {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn new(source: Option<Box<dyn Error>>, kind: ErrorKind) -> PlayerError {
        PlayerError { source, kind }
    }
}

impl PartialEq for PlayerError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in the MP3 Player of kind {:#?}", self.kind)
    }
}

impl Error for PlayerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&**self.source.as_ref().unwrap())
    }
}

impl From<std::io::Error> for PlayerError {
    fn from(error: std::io::Error) -> Self {
        PlayerError {
            source: Some(Box::new(error)),
            kind: ErrorKind::FileNotFound,
        }
    }
}

impl From<DecoderError> for PlayerError {
    fn from(error: DecoderError) -> Self {
        PlayerError {
            source: Some(Box::new(error)),
            kind: ErrorKind::RodioError,
        }
    }
}
