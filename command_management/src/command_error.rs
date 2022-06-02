use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    IoError,
    UnknownVerb,
    ParameterParsingFailed,
    ExecutionFailed,
    UnhandledCommand,
}

/// Error type that wraps error that can come from the command management, either its parsing, or its execution.
///
/// ### TODO
///
/// * Find a way to abstract the wrapping logic to reuse it somewhere else (ErrorKind enum as template type ?).
#[derive(Debug)]
pub struct CommandError {
    source: Option<Box<dyn Error>>,
    kind: ErrorKind,
    command: Option<String>,
    message: Option<String>,
}

impl CommandError {
    pub fn new(
        source: Option<Box<dyn Error>>,
        kind: ErrorKind,
        command: Option<String>,
        message: Option<String>,
    ) -> CommandError {
        CommandError {
            source,
            kind,
            command,
            message,
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in the command handling of kind {:#?}", self.kind)
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // WTF ?
        Some(&**self.source.as_ref().unwrap())
    }
}

impl From<std::io::Error> for CommandError {
    fn from(error: std::io::Error) -> Self {
        CommandError {
            source: Some(Box::new(error)),
            kind: ErrorKind::IoError,
            command: None,
            message: Some(String::from("Error while trying to read the command")),
        }
    }
}
