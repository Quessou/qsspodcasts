use std::cell::RefCell;
use std::io::stdout;
use std::sync::Arc;
use std::{error::Error, time::Duration};

use business_core::business_core::BusinessCore;
use command_management::output::output_type::OutputType;
use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use podcast_player::player_status::PlayerStatus;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::Instant;
use tui::{backend::CrosstermBackend, Terminal};

use command_management::command_engine::CommandEngine;
use podcast_player::mp3_player_exposer::Mp3PlayerExposer;

use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;
use crate::terminal_frontend_logger::TerminalFrontendLogger;
use crate::ui_drawers::ui_drawer::UiDrawer;

use tui::widgets::ListState;

/// TODO : Find a better name
enum ActionPostEvent {
    DoNothing,
    Quit,
}

pub struct Frontend<'a, D: UiDrawer> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_engine: Arc<TokioMutex<CommandEngine<'a>>>,
    context: ScreenContext,
    ui_drawer: Box<D>,
    mp3_player_exposer: Mp3PlayerExposer,
}

impl<D: UiDrawer> Frontend<'_, D> {
    pub fn new(
        business_core: BusinessCore,
        //mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        //podcast_library: Arc<TokioMutex<PodcastLibrary>>,
        ui_drawer: Box<D>,
    ) -> Frontend<'static, D> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        let context = ScreenContext::default();
        let mp3_player = business_core.player.clone();
        TerminalFrontendLogger::new(context.logs.clone())
            .init()
            .expect("Logger initialization failed");
        Frontend {
            terminal,
            command_engine: Arc::new(TokioMutex::new(CommandEngine::new(business_core))),
            context,
            ui_drawer,
            mp3_player_exposer: Mp3PlayerExposer::new(mp3_player),
        }
    }

    async fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<ActionPostEvent, Box<dyn Error>> {
        match self.context.current_action {
            ScreenAction::TypingCommand => match key_event.code {
                KeyCode::Enter => {
                    if self.context.command.is_empty() {
                        return Ok(ActionPostEvent::DoNothing);
                    }
                    if self.context.command.to_lowercase() == "exit" {
                        return Ok(ActionPostEvent::Quit);
                    }
                    let command = self.context.command.clone();
                    self.context.command = String::from("");

                    match self
                        .command_engine
                        .lock()
                        .await
                        .handle_command(&command)
                        .await
                    {
                        Err(_) => return Ok(ActionPostEvent::DoNothing),
                        Ok(s) => {
                            self.context.last_command_output = s.clone();
                            self.context.must_invalidate_cache.set(true);
                            if let OutputType::RawString(_) = s {
                                self.context.list_output_state = None;
                            } else {
                                self.context.list_output_state =
                                    Some(RefCell::new(ListState::default()));
                            }
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if c == '²' {
                        if self.context.last_command_output != OutputType::RawString("".to_string())
                        {
                            self.context.current_action = ScreenAction::ScrollingOutput;
                        }
                    } else {
                        self.context.command.push(c)
                    }
                }
                KeyCode::Backspace => {
                    self.context.command.pop();
                }
                KeyCode::Tab => (), // TODO : Handle auto-completion
                KeyCode::Delete => self.context.current_action = ScreenAction::ScrollingLogs,
                _ => (),
            },
            ScreenAction::ScrollingLogs => {
                if key_event.code == KeyCode::Delete {
                    self.context.current_action = ScreenAction::TypingCommand
                }
            }
            ScreenAction::ScrollingOutput => match key_event.code {
                KeyCode::Char(c) => {
                    if c == '²' {
                        self.context.current_action = ScreenAction::TypingCommand;
                    }
                }
                KeyCode::Down => {
                    if let Some(ref state) = self.context.list_output_state {
                        let output_length = match self.context.last_command_output {
                            OutputType::Episodes(ref v) => v.len(),
                            OutputType::Podcasts(ref v) => v.len(),
                            OutputType::RawString(_) => 0,
                        };

                        let mut state = state.borrow_mut();
                        let selected_index = match state.selected() {
                            Some(i) => (i + 1) % output_length,
                            None => 0,
                        };
                        state.select(Some(selected_index));
                    }
                }
                KeyCode::Up => {
                    if let Some(ref state) = self.context.list_output_state {
                        let output_length = match self.context.last_command_output {
                            OutputType::Episodes(ref v) => v.len(),
                            OutputType::Podcasts(ref v) => v.len(),
                            OutputType::RawString(_) => 0,
                        };

                        let mut state = state.borrow_mut();
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
                _ => (),
            },
        }

        Ok(ActionPostEvent::DoNothing)
    }

    async fn handle_event(
        &mut self,
        event: CrosstermEvent,
    ) -> Result<ActionPostEvent, Box<dyn Error>> {
        if let CrosstermEvent::Key(key_event) = event {
            return self.handle_key_event(key_event).await;
        }
        Ok(ActionPostEvent::DoNothing)
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

        let player_status = match player_exposer.is_paused().await {
            true => match player_exposer.get_selected_episode_progression().await {
                None => PlayerStatus::Stopped,
                Some(_) => PlayerStatus::Paused(
                    episode_progression.unwrap(),
                    episode_duration.unwrap(),
                    progression_percentage.unwrap(),
                ),
            },
            false => PlayerStatus::Playing(
                episode_progression.unwrap(),
                episode_duration.unwrap(),
                progression_percentage.unwrap(),
            ),
        };

        self.context.player_status = player_status;
        debug!("Screen context updated");
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            self.terminal
                .draw(|f| self.ui_drawer.draw_ui(f, &self.context))?;

            let timeout: Duration = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            debug!("polling for {} ms", timeout.as_millis());
            if crossterm::event::poll(timeout)? {
                let event = event::read().unwrap();
                if let ActionPostEvent::Quit = self.handle_event(event).await.unwrap() {
                    break;
                }
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
}
