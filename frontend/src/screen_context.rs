use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use command_management::output::output_type::OutputType;
use podcast_player::player_status::PlayerStatus;

use crate::screen_action::ScreenAction;

use tui::widgets::ListState;

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
/// How to prevent this struct from becoming a god class ?
pub struct ScreenContext {
    pub command: String,
    pub last_command_output: OutputType,
    pub list_output_state: Option<RefCell<ListState>>,
    pub previous_output_pane_available_width: Cell<Option<usize>>,
    pub must_invalidate_cache: Cell<bool>,

    pub logs: Arc<Mutex<Vec<String>>>,
    pub current_action: ScreenAction,
    pub ui_refresh_tickrate: Duration,
    pub player_status: PlayerStatus,
}

impl ScreenContext {
    pub fn get_output_list_length(&self) -> Option<usize> {
        match &self.last_command_output {
            OutputType::Episodes(l) => Some(l.len()),
            OutputType::Podcasts(l) => Some(l.len()),
            _ => None,
        }
    }
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            command: String::from(""),
            // Move all this output-related stuff in a struct called OutputContext
            last_command_output: OutputType::RawString(String::from("")),
            list_output_state: None,
            previous_output_pane_available_width: Cell::new(None),
            must_invalidate_cache: Cell::new(false),
            logs: Arc::new(Mutex::new(vec![])),
            current_action: ScreenAction::TypingCommand,
            ui_refresh_tickrate: Duration::from_millis(20),
            player_status: PlayerStatus::Stopped,
        }
    }
}
