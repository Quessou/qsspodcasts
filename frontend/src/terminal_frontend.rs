use std::io::stdout;
use std::sync::{mpsc::channel, Arc};
use std::thread;
use std::{error::Error, time::Duration};

use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;

use podcast_management::podcast_library::PodcastLibrary;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_status::PlayerStatus;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep as tokio_sleep;
use tui::{backend::CrosstermBackend, Terminal};

use command_management::command_engine::CommandEngine;
use podcast_player::{mp3_player_exposer::Mp3PlayerExposer, players::mp3_player::Mp3Player};

use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;
use crate::terminal_frontend_logger::TerminalFrontendLogger;
use crate::ui_drawers::ui_drawer::UiDrawer;

/// TODO : Find a better name
enum ActionPostEvent {
    DoNothing,
    Quit,
}

pub struct Frontend<'a, D: UiDrawer> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_engine: Arc<TokioMutex<CommandEngine>>,
    context: ScreenContext<'a>,
    ui_drawer: Box<D>,
    mp3_player_exposer: Mp3PlayerExposer,
}

impl<D: UiDrawer> Frontend<'_, D> {
    pub fn new(
        mp3_player: Arc<TokioMutex<dyn Mp3Player + Send>>,
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
        ui_drawer: Box<D>,
    ) -> Frontend<'static, D> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        let context = ScreenContext::default();
        TerminalFrontendLogger::new(context.logs.clone())
            .init()
            .expect("Logger initialization failed");
        Frontend {
            terminal,
            command_engine: Arc::new(TokioMutex::new(CommandEngine::new(
                mp3_player.clone(),
                podcast_library,
            ))),
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
                            self.context.last_formatted_command_output = s.into();
                            self.context.output_index = Some(0);

                            //if output_overflow()
                            // TODO : Transition to ScrollingLogs state if output is bigger than the pane (really ?)
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if c == '²' {
                        self.context.current_action = ScreenAction::ScrollingOutput;
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
                _ => (),
            },

            _ => (),
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
                    episode_progression.unwrap().to_string(),
                    episode_duration.unwrap().to_string(),
                    progression_percentage.unwrap(),
                ),
            },
            false => PlayerStatus::Playing(
                episode_progression.unwrap().to_string(),
                episode_duration.unwrap().to_string(),
                progression_percentage.unwrap(),
            ),
        };

        self.context.player_status = player_status;
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;

        let (tx, rx) = channel();
        thread::spawn(move || {
            let input_polling_rate = Duration::from_millis(50);
            log::info!("Launching input polling thread");
            loop {
                if crossterm::event::poll(input_polling_rate).unwrap() {
                    let event = Some(event::read().unwrap());
                    if let Err(e) = tx.send(event) {
                        log::error!("Send error while sending event {e}");
                    }
                } else {
                    tx.send(None).unwrap();
                }
            }
        });

        loop {
            self.terminal
                .draw(|f| self.ui_drawer.draw_ui(f, &self.context))?;
            if let Some(event) = rx.recv().unwrap() {
                if let ActionPostEvent::Quit = self.handle_event(event).await.unwrap() {
                    break;
                }
            } else {
                tokio_sleep(self.context.ui_refresh_tickrate).await;
                self.update_screen_context().await;
            }
        }
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
