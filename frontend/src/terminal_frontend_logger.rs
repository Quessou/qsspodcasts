use log::{self, Log, Metadata, Record, SetLoggerError};

use std::sync::Mutex;

pub struct TerminalFrontendLogger {
    pub log: Mutex<Vec<String>>,
}

impl TerminalFrontendLogger {
    #[must_use = "You must call init() to begin logging"]
    pub fn new() -> TerminalFrontendLogger {
        TerminalFrontendLogger {
            log: Mutex::new(vec![]),
        }
    }

    pub fn init(mut self) -> Result<(), SetLoggerError> {
        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Default for TerminalFrontendLogger {
    fn default() -> TerminalFrontendLogger {
        TerminalFrontendLogger::new()
    }
}

impl Log for TerminalFrontendLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // TODO : Take care of internal mutability here
        self.log.lock().unwrap().push(String::from("Blbl"));
    }

    fn flush(&self) {}
}
