use std::cell::RefCell;
use std::io::stdout;
use std::sync::Arc;
use std::{error::Error, time::Duration};

use command_management::output::output_type::OutputType;
use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, error};
use podcast_player::player_status::PlayerStatus;
use podcast_player::players::mp3_player::Mp3Player;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::Instant;
use tui::{backend::CrosstermBackend, Terminal};

use business_core::notification::Notification;
use podcast_player::mp3_player_exposer::Mp3PlayerExposer;

use crate::crossterm_async_event::poll;
use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;
use crate::terminal_frontend_logger::TerminalFrontendLogger;
use crate::ui_drawers::ui_drawer::UiDrawer;
use command_management::command_engine::CommandResult;
use data_transport::{data_receiver::DataReceiver, data_sender::DataSender};
use tui::widgets::ListState;

pub struct Frontend<D: UiDrawer> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_sender: DataSender<String>,
    output_receiver: DataReceiver<CommandResult>,
    notification_receiver: DataReceiver<Notification>,
    context: ScreenContext,
    ui_drawer: Box<D>,
    mp3_player_exposer: Mp3PlayerExposer,
}

impl<D: UiDrawer> Frontend<D> {
    pub fn new(
        command_sender: DataSender<String>,
        output_receiver: DataReceiver<CommandResult>,
        notification_receiver: DataReceiver<Notification>,
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        ui_drawer: Box<D>,
    ) -> Frontend<D> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        let context = ScreenContext::default();
        TerminalFrontendLogger::new(context.logs.clone())
            .init()
            .expect("Logger initialization failed");
        Frontend {
            terminal,
            command_sender,
            output_receiver,
            notification_receiver,
            context,
            ui_drawer,
            mp3_player_exposer: Mp3PlayerExposer::new(mp3_player),
        }
    }

    fn handle_output(&mut self, output: OutputType) {
        if output != OutputType::None {
            self.context.last_command_output = output.clone();
            self.context.must_invalidate_cache.set(true);
            if let OutputType::RawString(_) = output {
                self.context.list_output_state = None;
            } else {
                self.context.list_output_state = Some(RefCell::new(ListState::default()));
            }
        }
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), Box<dyn Error>> {
        match self.context.current_action {
            ScreenAction::TypingCommand => match key_event.code {
                KeyCode::Enter => {
                    if self.context.command.is_empty() {
                        return Ok(());
                    }

                    let command = self.context.command.clone();
                    self.context.command = String::from("");
                    if let Err(e) = self.command_sender.send(command).await {
                        error!("Could not send command.");
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
                            _ => 0,
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
                            _ => 0,
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
            let r = poll(timeout).await;
            if r.is_ok() && r.unwrap() {
                let event = event::read().unwrap();
                if self.handle_event(event).await.is_err() {
                    // TODO
                }
            } else if r.is_err() {
                error!("Error while handling incoming crossterm event")
            }

            if let Ok(output) = self.output_receiver.try_receive() {
                if let Ok(o) = output {
                    self.handle_output(o)
                }
            }

            if let Ok(n) = self.notification_receiver.try_receive() {
                self.context.notifications_buffer.push_front(n);
                self.context.notifications_buffer.truncate(5);
            }

            if self.command_sender.is_closed() {
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
}
