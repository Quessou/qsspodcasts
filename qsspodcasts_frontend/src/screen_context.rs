use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use command_management::output::output_type::OutputType;
use data_caches::PodcastStateCache;
use podcast_management::data_objects::hashable::Hashable;
use podcast_player::player_status::PlayerStatus;

use crate::autocompletion_context::AutocompletionContext;
use crate::modal_window::action_list_builder::ActionListBuilder;
use crate::modal_window::interactable_modal_window_context::InteractableModalWindowContext;
use crate::modal_window::modal_action::ModalAction;
use crate::modal_window::readonly_modal_context::ReadonlyModalContext;
use crate::screen_action::ScreenAction;

use tui::widgets::ListState;

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
/// How to prevent this struct from becoming a god class ?
pub struct ScreenContext {
    pub(crate) last_command_output: OutputType,
    pub(crate) list_output_state: Option<RefCell<ListState>>,
    pub(crate) previous_output_pane_available_width: Cell<Option<usize>>,
    pub(crate) must_invalidate_cache: Cell<bool>,
    pub(crate) logs: Arc<Mutex<Vec<String>>>,
    pub(crate) current_action: ScreenAction,
    pub(crate) stacked_states: Vec<ScreenAction>,
    pub(crate) ui_refresh_tickrate: Duration,
    pub(crate) player_status: PlayerStatus,
    pub(crate) message_notifications_buffer: VecDeque<String>,
    pub(crate) autocompletion_context: AutocompletionContext,
    pub(crate) interactable_modal_context: InteractableModalWindowContext,
    pub(crate) read_only_modal_context: ReadonlyModalContext,
    pub(crate) podcasts_state_cache: PodcastStateCache,
    pub(crate) current_podcast_title: Option<String>,
    pub(crate) volume: u32,
}

impl ScreenContext {
    pub fn build(podcasts_state_cache: PodcastStateCache) -> Self {
        Self {
            podcasts_state_cache,
            ..Default::default()
        }
    }

    pub fn get_output_list_length(&self) -> Option<usize> {
        match &self.last_command_output {
            OutputType::Episodes(l) => Some(l.len()),
            OutputType::Podcasts(l) => Some(l.len()),
            OutputType::CommandHelps(l) => Some(l.len()),
            _ => None,
        }
    }

    pub fn get_element_modal_actions_data(
        &self,
        index: usize,
        builder: &ActionListBuilder, // TODO : Kinda bad to have to pass a builder as a parameter
    ) -> Vec<ModalAction> {
        match self.last_command_output {
            OutputType::Episodes(ref v) => builder.build_action_list(&v[index]),
            OutputType::Podcasts(ref v) => builder.build_action_list(&v[index]),
            OutputType::CommandHelps(ref v) => builder.build_action_list(&v[index]),
            _ => unreachable!(),
        }
    }

    pub fn pop_previous_state(&mut self) -> Option<ScreenAction> {
        self.stacked_states.pop()
    }

    /// Updates context when a podcast whose hash is given in parameter is finished
    /// This can induce an cached output invalidation and a recomputing of it, to display the fact
    /// that the current podcast is finished
    pub fn on_podcast_finished(&mut self, hash: &str) {
        if let OutputType::Episodes(ref episodes) = self.last_command_output {
            let matching_episodes = episodes.iter().filter(|e| e.hash() == hash);
            assert_eq!(matching_episodes.clone().count(), 1);
            // TODO: Mark the podcast as finished ? Not here I think
        }
    }
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            // TODO: Move all this output-related stuff in a struct called OutputContext
            last_command_output: OutputType::RawString(String::from("")),
            list_output_state: None,
            previous_output_pane_available_width: Cell::new(None),
            must_invalidate_cache: Cell::new(false),
            logs: Arc::new(Mutex::new(vec![])),
            current_action: ScreenAction::TypingCommand,
            stacked_states: vec![],
            ui_refresh_tickrate: Duration::from_millis(250),
            player_status: PlayerStatus::Stopped(None),
            message_notifications_buffer: VecDeque::with_capacity(4),
            autocompletion_context: AutocompletionContext::default(),
            interactable_modal_context: InteractableModalWindowContext::default(),
            read_only_modal_context: ReadonlyModalContext::default(),
            podcasts_state_cache: PodcastStateCache::default(),
            current_podcast_title: None,
            volume: 0,
        }
    }
}
