use tui::{
    backend::{Backend, CrosstermBackend},
    widgets, Frame, Terminal,
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use std::error::Error;
use std::io::stdout;

pub struct Frontend {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Frontend {
    pub fn new() -> Frontend {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        Frontend { terminal }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        loop {
            self.terminal.draw(|f| Frontend::draw_ui(f))?;
            // TODO : Handle exit of the UI
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn draw_ui<B: Backend>(f: &mut Frame<B>) {
        let size = f.size();
        //let tutu = widgets::Paragraph {};
    }
}
