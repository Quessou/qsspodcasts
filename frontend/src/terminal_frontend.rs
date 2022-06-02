use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::{error::Error, time::Duration};

use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use podcast_management::podcast_library::PodcastLibrary;
use tokio::sync::Mutex as TokioMutex;
use tui::{backend::CrosstermBackend, Terminal};

use command_management::command_engine::CommandEngine;
use podcast_player::mp3_player::Mp3Player;

use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;
use crate::terminal_frontend_logger::TerminalFrontendLogger;
use crate::ui_drawers::ui_drawer::UiDrawer;

/// TODO : Find a better name
enum ActionPostEvent {
    DoNothing,
    Quit,
}

pub struct Frontend<D: UiDrawer> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_engine: Arc<TokioMutex<CommandEngine>>,
    context: ScreenContext,
    ui_drawer: Box<D>,
}

impl<D: UiDrawer> Frontend<D> {
    pub fn new(
        mp3_player: Arc<TokioMutex<Mp3Player>>,
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
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
            command_engine: Arc::new(TokioMutex::new(CommandEngine::new(
                mp3_player,
                podcast_library,
            ))),
            context: context,
            ui_drawer,
        }
    }

    async fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<ActionPostEvent, Box<dyn Error>> {
        match self.context.current_action {
            ScreenAction::TypingCommand => match key_event.code {
                KeyCode::Enter => {
                    if self.context.command.len() == 0 {
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
                        Ok(s) => self.context.last_command_output = s,
                    }
                }
                KeyCode::Char(c) => self.context.command.push(c),
                KeyCode::Backspace => {
                    self.context.command.pop();
                }
                KeyCode::Tab => (), // TODO : Handle auto-completion
                KeyCode::Delete => self.context.current_action = ScreenAction::ScrollingLogs,
                _ => (),
            },
            ScreenAction::ScrollingLogs => match key_event.code {
                KeyCode::Delete => self.context.current_action = ScreenAction::TypingCommand,
                _ => (),
            },
            _ => (),
        }
        Ok(ActionPostEvent::DoNothing)
    }

    async fn handle_event(&mut self) -> Result<ActionPostEvent, Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            return self.handle_key_event(key).await;
        }
        Ok(ActionPostEvent::DoNothing)
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;
        loop {
            self.terminal
                .draw(|f| self.ui_drawer.draw_ui(f, &self.context))?;

            if crossterm::event::poll(self.context.ui_refresh_tickrate)? {
                if let ActionPostEvent::Quit = self.handle_event().await.unwrap() {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
