use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::screen_action::ScreenAction;

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
pub struct ScreenContext {
    pub command: String,
    pub last_command_output: String,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub current_action: ScreenAction,
    pub ui_refresh_tickrate: Duration,
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            command: String::from(""),
            last_command_output: String::from(""),
            logs: Arc::new(Mutex::new(vec![])),
            current_action: ScreenAction::TypingCommand,
            ui_refresh_tickrate: Duration::from_millis(200),
        }
    }
}
