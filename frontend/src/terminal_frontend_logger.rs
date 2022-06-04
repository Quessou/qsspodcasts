use log::{self, Log, Metadata, Record, SetLoggerError};

use std::sync::{Arc, Mutex};

pub struct TerminalFrontendLogger {
    pub log_buffer: Arc<Mutex<Vec<String>>>,
}

impl TerminalFrontendLogger {
    #[must_use = "You must call init() to begin logging"]
    pub fn new(log_buffer: Arc<Mutex<Vec<String>>>) -> TerminalFrontendLogger {
        TerminalFrontendLogger { log_buffer }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(log::LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Log for TerminalFrontendLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        self.log_buffer
            .lock()
            .unwrap()
            .push(format!("{}", record.args()));
    }

    fn flush(&self) {}
}
