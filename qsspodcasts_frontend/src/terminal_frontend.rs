use std::cell::RefCell;
use std::io::stdout;
use std::sync::Arc;
use std::{error::Error, time::Duration};

use business_core::event_type::EventType;
use command_management::commands::command_enum::Command;
use command_management::output::output_type::OutputType;
use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use data_caches::PodcastStateCache;
use log::{debug, error};
use podcast_management::data_objects::podcast_state::PodcastState;
use podcast_player::enums::player_state::Mp3PlayerState;
use podcast_player::player_status::PlayerStatus;
use podcast_player::players::mp3_player::Mp3Player;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::Instant;
use tui::{backend::CrosstermBackend, Terminal};

use autocomplete_server::AutocompletionResponse;
use business_core::notification::Notification;
use podcast_player::mp3_player_exposer::Mp3PlayerExposer;

use crate::crossterm_async_event::poll;
use crate::modal_window::action_list_builder::ActionListBuilder;
use crate::modal_window::read_only_modal_contents::first_launch_help::{
    self, get_first_launch_help,
};
use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;
use crate::terminal_frontend_logger::TerminalFrontendLogger;
use crate::ui_drawers::ui_drawer::UiDrawer;
use command_management::command_engine::CommandResult;
use data_transport::{AutocompleterMessageType, DataReceiver, DataSender};
use tui::widgets::ListState;

pub struct Frontend<D: UiDrawer> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_sender: DataSender<String>,
    output_receiver: DataReceiver<CommandResult>,
    autocompletion_request_sender: DataSender<AutocompleterMessageType>,
    autocompletion_response_reader: DataReceiver<AutocompletionResponse>,
    notification_receiver: DataReceiver<Notification>,
    context: ScreenContext,
    ui_drawer: Box<D>,
    mp3_player_exposer: Mp3PlayerExposer,
    action_list_builder: ActionListBuilder,
}

impl<D: UiDrawer> Frontend<D> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        command_sender: DataSender<String>,
        output_receiver: DataReceiver<CommandResult>,
        notification_receiver: DataReceiver<Notification>,
        autocompletion_request_sender: DataSender<AutocompleterMessageType>,
        autocompletion_response_reader: DataReceiver<AutocompletionResponse>,
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send + Sync>>,
        ui_drawer: Box<D>,
        podcast_state_cache: PodcastStateCache,
    ) -> Frontend<D> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        let context = ScreenContext::build(podcast_state_cache);
        TerminalFrontendLogger::new(context.logs.clone())
            .init()
            .expect("Logger initialization failed");
        Frontend {
            terminal,
            command_sender: command_sender.clone(),
            output_receiver,
            notification_receiver,
            autocompletion_request_sender,
            autocompletion_response_reader,
            context,
            ui_drawer,
            mp3_player_exposer: Mp3PlayerExposer::new(mp3_player),
            action_list_builder: ActionListBuilder::new(command_sender),
        }
    }

    fn handle_output(&mut self, output: OutputType) {
        if output != OutputType::None {
            self.context.last_command_output = output.clone();
            self.context.must_invalidate_cache.set(true);
            if let OutputType::RawString(_) = output {
                self.context.list_output_state = None;
            } else {
                self.context.list_output_state =
                    Some(RefCell::new(ListState::default().with_selected(Some(0))));
            }
        }
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), Box<dyn Error>> {
        match self.context.current_action {
            ScreenAction::TypingCommand => match key_event.code {
                KeyCode::Enter => {
                    let autoctxt = &mut self.context.autocompletion_context;
                    if autoctxt.current_input.is_empty() {
                        return Ok(());
                    }

                    // This condition sucks AF
                    if autoctxt.is_autocompletion_buffer_empty()
                        || autoctxt.is_autocompletion_request_required()
                    {
                        let command = autoctxt.current_input.clone();
                        autoctxt.current_input = String::from("");
                        if self.command_sender.send(command).await.is_err() {
                            error!("Could not send command.");
                        }
                        autoctxt.reset();
                    } else if !autoctxt.is_autocompletion_buffer_empty() {
                        autoctxt.confirm();
                    }
                }
                KeyCode::Right => {
                    let autoctxt = &mut self.context.autocompletion_context;
                    if !autoctxt.is_autocompletion_buffer_empty() {
                        autoctxt.confirm();
                    }
                }
                KeyCode::Tab => {
                    let autocompletion_ctxt = &mut self.context.autocompletion_context;
                    if autocompletion_ctxt.is_autocompletion_request_possible()
                        && autocompletion_ctxt.is_autocompletion_request_required()
                    {
                        self.autocompletion_request_sender
                            .send(AutocompleterMessageType::AutocompletionRequest(
                                autocompletion_ctxt.current_input.clone(),
                            ))
                            .await
                            .expect("Sending of autocompletion request failed");
                        let autocompletion_response =
                            self.autocompletion_response_reader.receive().await.unwrap();
                        autocompletion_ctxt.set_autocompletion_choices(
                            autocompletion_response.autocompletion_options,
                        );
                    } else if autocompletion_ctxt.current_choice.is_some() {
                        autocompletion_ctxt.go_to_next_choice();
                    }
                    // When to reset ?
                }
                KeyCode::Char(c) => {
                    if c == '²' {
                        if self.context.last_command_output != OutputType::RawString("".to_string())
                        {
                            self.context.current_action = ScreenAction::ScrollingOutput;
                        }
                    } else {
                        self.context.autocompletion_context.current_input.push(c);
                        if self.context.autocompletion_context.is_ctxt_initialized() {
                            self.context.autocompletion_context.push_current_state();
                            self.context.autocompletion_context.narrow_choices();
                        }
                    }
                }
                KeyCode::Backspace => {
                    self.context.autocompletion_context.current_input.pop();
                    if self.context.autocompletion_context.is_ctxt_initialized() {
                        self.context.autocompletion_context.pop_state();
                    }
                }
                KeyCode::Delete => self.context.current_action = ScreenAction::ScrollingLogs,
                KeyCode::F(1) => {
                    self.context
                        .stacked_states
                        .push(self.context.current_action);
                    self.context.current_action = ScreenAction::ShowingReadOnlyModalWindow;
                    self.context.read_only_modal_context.content =
                        Some(first_launch_help::get_first_launch_help());
                }
                _ => (),
            },
            ScreenAction::ScrollingLogs => {
                if key_event.code == KeyCode::Delete {
                    self.context.current_action = ScreenAction::TypingCommand
                }
            }
            ScreenAction::ScrollingOutput => match key_event.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(ref state) = self.context.list_output_state {
                        let output_length = match self.context.last_command_output {
                            OutputType::Episodes(ref v) => v.len(),
                            OutputType::Podcasts(ref v) => v.len(),
                            OutputType::CommandHelps(ref v) => v.len(),
                            _ => 0,
                        };

                        let mut state = state.borrow_mut();
                        if state.selected().is_none() {
                            return Ok(());
                        }
                        let selected_index = match state.selected() {
                            Some(i) => (i + 1) % output_length,
                            None => 0,
                        };
                        state.select(Some(selected_index));
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(ref state) = self.context.list_output_state {
                        let output_length = match self.context.last_command_output {
                            OutputType::Episodes(ref v) => v.len(),
                            OutputType::Podcasts(ref v) => v.len(),
                            OutputType::CommandHelps(ref v) => v.len(),
                            _ => 0,
                        };
                        // TODO : Mutualize this properly
                        let mut state = state.borrow_mut();
                        if state.selected().is_none() {
                            return Ok(());
                        }
                        let selected_index = match state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    output_length - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(selected_index));
                    }
                }
                KeyCode::Enter => {
                    self.context.current_action = ScreenAction::ScrollingModalWindow;
                    let selected_index = self
                        .context
                        .list_output_state
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .selected();
                    if selected_index.is_none() {
                        return Ok(());
                    }
                    let selected_index = selected_index.unwrap();
                    let actions = self
                        .context
                        .get_element_modal_actions_data(selected_index, &self.action_list_builder);
                    self.context.interactable_modal_context.reset(Some(actions));
                }
                // TODO(mmiko) : Replace this reference to ² by something else tbd
                KeyCode::Char('²') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.context.current_action = ScreenAction::TypingCommand;
                }
                KeyCode::F(1) => {
                    self.context
                        .stacked_states
                        .push(self.context.current_action);
                    self.context.current_action = ScreenAction::ShowingReadOnlyModalWindow;
                    self.context.read_only_modal_context.content =
                        Some(first_launch_help::get_first_launch_help());
                }
                _ => (),
            },
            ScreenAction::ScrollingModalWindow => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.context.current_action = ScreenAction::ScrollingOutput
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    // TODO : Mutualize
                    let mut state = self
                        .context
                        .interactable_modal_context
                        .modal_actions_list_state
                        .as_ref()
                        .unwrap()
                        .borrow_mut();
                    if state.selected().is_none() {
                        return Ok(());
                    }
                    let actions_list_length = self
                        .context
                        .interactable_modal_context
                        .modal_actions
                        .as_ref()
                        .unwrap()
                        .len();
                    let selected_index = match state.selected() {
                        Some(i) => {
                            if i == 0 {
                                actions_list_length - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    state.select(Some(selected_index));
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    // TODO : Mutualize
                    let mut state = self
                        .context
                        .interactable_modal_context
                        .modal_actions_list_state
                        .as_ref()
                        .unwrap()
                        .borrow_mut();
                    if state.selected().is_none() {
                        return Ok(());
                    }
                    let actions_list_length = self
                        .context
                        .interactable_modal_context
                        .modal_actions
                        .as_ref()
                        .unwrap()
                        .len();
                    if state.selected().is_none() {
                        return Ok(());
                    }

                    let selected_index = match state.selected() {
                        Some(i) => (i + 1) % actions_list_length,
                        None => 0,
                    };
                    state.select(Some(selected_index));
                }
                KeyCode::Enter => {
                    let actions = self
                        .context
                        .interactable_modal_context
                        .modal_actions
                        .as_mut()
                        .unwrap();
                    let selected_index = self
                        .context
                        .interactable_modal_context
                        .modal_actions_list_state
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .selected();
                    if selected_index.is_none() {
                        return Ok(());
                    }
                    let selected_index = selected_index.unwrap();
                    if (actions[selected_index].call().await).is_err() {
                        panic!("Execution of modal action failed")
                    }
                    self.context.current_action = ScreenAction::TypingCommand;
                    self.context.autocompletion_context.clear();
                }
                _ => {}
            },
            ScreenAction::ShowingReadOnlyModalWindow => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::F(1) => {
                    let prev_state = self.context.pop_previous_state();
                    self.context.current_action = prev_state.unwrap();
                }
                _ => {}
            },
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: CrosstermEvent) -> Result<(), Box<dyn Error>> {
        if let CrosstermEvent::Key(key_event) = event {
            return self.handle_key_event(key_event).await;
        }
        Ok(())
    }

    /// Updates the screen context according to the entire system state (Mp3Player, and so on...)
    async fn update_screen_context(&mut self) {
        // Updating player_status
        let player_exposer = &self.mp3_player_exposer;
        let episode_progression = player_exposer.get_selected_episode_progression().await;
        let episode_duration = player_exposer.get_selected_episode_duration().await;
        let progression_percentage = player_exposer
            .get_selected_episode_progression_percentage()
            .await;

        let player_status = match player_exposer.get_state().await {
            Mp3PlayerState::Paused => match episode_progression {
                None => panic!(),
                Some(_) => PlayerStatus::Paused(
                    episode_progression.unwrap(),
                    episode_duration.unwrap(),
                    progression_percentage.unwrap(),
                ),
            },
            Mp3PlayerState::Playing => PlayerStatus::Playing(
                episode_progression.unwrap(),
                episode_duration.unwrap(),
                progression_percentage.unwrap(),
            ),
            Mp3PlayerState::Stopped => match episode_progression {
                Some(_) => PlayerStatus::Stopped(Some((
                    episode_progression.unwrap(),
                    episode_duration.unwrap(),
                    progression_percentage.unwrap(),
                ))),
                None => PlayerStatus::Stopped(None),
            },
            Mp3PlayerState::Buffering => PlayerStatus::Stopped(None),
        };

        self.context.player_status = player_status;
        debug!("Screen context updated");
    }

    pub async fn run(&mut self, first_start: bool) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;

        self.command_sender
            .send(Command::ListPodcasts.to_string())
            .await
            .unwrap();
        self.context
            .stacked_states
            .push(self.context.current_action);
        if first_start {
            self.context.current_action = ScreenAction::ShowingReadOnlyModalWindow;
            self.context.read_only_modal_context.content = Some(get_first_launch_help());
        }

        let tick_rate = self.context.ui_refresh_tickrate;
        let mut last_tick = Instant::now();
        loop {
            self.terminal
                .draw(|f| self.ui_drawer.draw_ui(f, &self.context))?;

            let timeout: Duration = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            debug!("polling for {} ms", timeout.as_millis());
            let r = poll(timeout).await;
            if r.is_ok() && r.unwrap() {
                let event = event::read().unwrap();
                if self.handle_event(event).await.is_err() {
                    // TODO
                }
            } else if r.is_err() {
                error!("Error while handling incoming crossterm event")
            }

            if let Ok(Ok(output)) = self.output_receiver.try_receive() {
                self.handle_output(output);
            }

            if let Ok(n) = self.notification_receiver.try_receive() {
                self.handle_business_notification(n);
            }

            if self.command_sender.is_closed() {
                self.autocompletion_request_sender
                    .send(AutocompleterMessageType::Exit)
                    .await
                    .expect("Sending of exit command to autocompleter failed");
                self.autocompletion_response_reader.close();
                break;
            }

            self.update_screen_context().await;
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_business_notification(&mut self, notification: Notification) {
        use EventType::*;
        #[allow(unreachable_patterns)]
        match notification {
            Notification::Message(m) => {
                self.context.message_notifications_buffer.push_front(m);
                // TODO : Put the 4 in a constant variable called "notification_window_height"
                self.context.message_notifications_buffer.truncate(4);
            }
            Notification::Event(e) => match e {
                PodcastFinished(hash) => {
                    self.context
                        .podcasts_state_cache
                        .set_podcast_state(&hash, &PodcastState::Finished);
                    // TODO: Add a condition to invalidate cache
                    self.context.must_invalidate_cache.set(true);
                }
                business_core::event_type::EventType::PodcastLaunched(title) => {
                    self.context.current_podcast_title = Some(title)
                }
                _ => {
                    error!("Received unhandled event {:?}", e);
                }
            },
        }
    }
}
