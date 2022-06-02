use std::io::stdout;
use std::sync::Arc;
use std::{error::Error, time::Duration};

use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use podcast_management::podcast_library::PodcastLibrary;
use tokio::sync::Mutex as TokioMutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{self, Block, Borders, Paragraph},
    Frame, Terminal,
};

use command_management::command_engine::CommandEngine;
use podcast_player::mp3_player::Mp3Player;

use crate::terminal_frontend_logger::TerminalFrontendLogger;

enum ScreenAction {
    TypingCommand,
    ScrollingOutput,
    ScrollingLogs,
}

/// TODO : Find a better name
enum ActionPostEvent {
    DoNothing,
    Quit,
}

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
struct ScreenContext {
    pub command: String,
    pub last_command_output: String,
    pub logs: String,
    pub current_action: ScreenAction,
    pub ui_refresh_tickrate: Duration,
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            command: String::from(""),
            last_command_output: String::from(""),
            logs: String::from(""),
            current_action: ScreenAction::TypingCommand,
            ui_refresh_tickrate: Duration::from_millis(200),
        }
    }
}

pub struct Frontend {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_engine: Arc<TokioMutex<CommandEngine>>,
    context: ScreenContext,
}

impl Frontend {
    pub fn new(
        mp3_player: Arc<TokioMutex<Mp3Player>>,
        podcast_library: Arc<TokioMutex<PodcastLibrary>>,
    ) -> Frontend {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        TerminalFrontendLogger::default()
            .init()
            .expect("Logger initialization failed");
        Frontend {
            terminal,
            command_engine: Arc::new(TokioMutex::new(CommandEngine::new(
                mp3_player,
                podcast_library,
            ))),
            context: ScreenContext::default(),
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
                .draw(|f| Frontend::draw_ui(f, &self.context))?;

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

    fn draw_ui<B: Backend>(f: &mut Frame<B>, context: &ScreenContext) {
        let size = f.size();

        // Defining screen layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(f.size());

        let input = Paragraph::new(context.command.as_ref())
            .style(match context.current_action {
                ScreenAction::TypingCommand => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Command"));
        f.render_widget(input, chunks[0]);

        let output = Paragraph::new(context.last_command_output.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Output"));
        f.render_widget(output, chunks[1]);
    }
}
