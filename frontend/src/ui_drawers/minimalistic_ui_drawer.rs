use podcast_player::player_status::PlayerStatus;
use tui::backend::Backend;
use tui::layout::Corner;
use tui::text::{Span, Spans};
use tui::widgets::ListItem;
use tui::Frame;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, Paragraph},
};

use command_management::output::command_output::CommandOutput;
use podcast_management::style::stylized::Stylized;

use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;

use super::ui_drawer;

pub struct MinimalisticUiDrawer {}

impl MinimalisticUiDrawer {
    pub fn new() -> MinimalisticUiDrawer {
        MinimalisticUiDrawer {}
    }

    fn draw_log_screen<B: Backend>(&self, f: &mut Frame<B>, context: &ScreenContext) {
        let size = f.size();
        let chunk = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(1)].as_ref())
            .split(size)[0];

        let logs = context.logs.lock().unwrap();
        let logs_list: Vec<ListItem> = logs
            .iter()
            .rev()
            .map(|s| ListItem::new(Spans::from(vec![Span::raw(s)])))
            .collect();

        let log_output = List::new(logs_list)
            .block(Block::default().borders(Borders::ALL).title("Log output"))
            .start_corner(Corner::BottomLeft);
        f.render_widget(log_output, chunk);
    }

    fn draw_main_screen<B: Backend>(&self, f: &mut Frame<B>, context: &ScreenContext) {
        let size = f.size();

        // Defining screen layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(size);

        let input = Paragraph::new(context.command.as_ref())
            .style(match context.current_action {
                ScreenAction::TypingCommand => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(vec![Span::from("Command")]),
            );
        f.render_widget(input, chunks[0]);

        let status = &context.player_status;
        let empty_string = String::from(""); // Isn't there something better to do here ?
        let (progress, duration, percentage) = match status {
            PlayerStatus::Playing(prog, dur, perc) => (prog, dur, *perc),
            PlayerStatus::Paused(prog, dur, perc) => (prog, dur, *perc),
            PlayerStatus::Stopped => (&empty_string, &empty_string, 0),
        };

        let podcast_progress = Gauge::default()
            .block(Block::default().title("").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::LightYellow))
            .label(format!("{progress}/{duration}"))
            .percent(percentage.into());
        f.render_widget(podcast_progress, chunks[1]);

        // TODO : Rework that
        let output = Paragraph::new(context.last_command_output.to_stylized()[0].0)
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Output"));
        f.render_widget(output, chunks[2]);
    }
}

impl ui_drawer::UiDrawer for MinimalisticUiDrawer {
    fn draw_ui<B: Backend>(&self, f: &mut Frame<B>, context: &ScreenContext) {
        match context.current_action {
            ScreenAction::ScrollingLogs => self.draw_log_screen(f, context),
            _ => self.draw_main_screen(f, context),
        }
    }
}
impl Default for MinimalisticUiDrawer {
    fn default() -> Self {
        Self::new()
    }
}
