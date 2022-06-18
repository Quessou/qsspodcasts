use std::sync::{Arc, Mutex};
use std::time::Duration;

use command_management::output::command_output::CommandOutput;
use podcast_player::player_status::PlayerStatus;

use crate::screen_action::ScreenAction;

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
/// How to prevent this struct from becoming a god class ?
pub struct ScreenContext {
    pub command: String,
    // TODO : Replace by Option<CommandOutput>
    pub last_command_output: CommandOutput,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub current_action: ScreenAction,
    pub ui_refresh_tickrate: Duration,
    pub player_status: PlayerStatus,
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            command: String::from(""),
            last_command_output: CommandOutput::new(vec![Box::new(String::from(""))]),
            logs: Arc::new(Mutex::new(vec![])),
            current_action: ScreenAction::TypingCommand,
            ui_refresh_tickrate: Duration::from_millis(20),
            player_status: PlayerStatus::Stopped,
        }
    }
}
