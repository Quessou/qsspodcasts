use std::error::Error;
use std::fmt;

use rodio::decoder::DecoderError;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    FileNotFound,
    RodioError,
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
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in the MP3 Player")
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
