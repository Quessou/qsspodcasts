use std::io::stdout;
use std::sync::Arc;
use std::{error::Error, time::Duration};

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

/// TODO : Find a better way to store these data ?
/// Page system for logs & outputs ?
/// Screen height ?
struct ScreenContext {
    pub command: String,
    pub last_command_output: String,
    pub logs: String,
    pub current_action: ScreenAction,
    pub toto: u32,
}

impl Default for ScreenContext {
    fn default() -> Self {
        ScreenContext {
            command: String::from(""),
            last_command_output: String::from(""),
            logs: String::from(""),
            current_action: ScreenAction::TypingCommand,
            toto: 0,
        }
    }
}

pub struct Frontend {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    command_engine: Arc<TokioMutex<CommandEngine>>,
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
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ctxt = ScreenContext::default();
        enable_raw_mode()?;
        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;
        loop {
            self.terminal.draw(|f| Frontend::draw_ui(f, &mut ctxt))?;

            if crossterm::event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match ctxt.current_action {
                        ScreenAction::TypingCommand => match key.code {
                            KeyCode::Enter => {
                                if ctxt.command.len() == 0 {
                                    continue;
                                }
                                if ctxt.command.to_lowercase() == "exit" {
                                    break;
                                }
                                let command = ctxt.command;
                                ctxt.command = String::from("");

                                match self
                                    .command_engine
                                    .lock()
                                    .await
                                    .handle_command(&command)
                                    .await
                                {
                                    Err(_) => continue,
                                    Ok(s) => ctxt.last_command_output = s,
                                }
                            }
                            KeyCode::Char(c) => ctxt.command.push(c),
                            KeyCode::Backspace => {
                                ctxt.command.pop();
                                ()
                            }
                            KeyCode::Tab => (), // TODO : Handle auto-completion
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
        }
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw_ui<B: Backend>(f: &mut Frame<B>, context: &mut ScreenContext) {
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
