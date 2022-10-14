use command_management::output::output_type::OutputType;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_status::PlayerStatus;
use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::Frame;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};

use crate::screen_action::ScreenAction;
use crate::screen_context::ScreenContext;

use super::output_management::vec_list_items::{
    build_list_item_from_episode, build_list_item_from_podcast,
};
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

    fn build_screen_layout(_context: &ScreenContext, size: &Rect) -> Vec<Rect> {
        Layout::default()
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
            .split(*size)
    }

    fn build_input_field(context: &ScreenContext) -> Paragraph {
        Paragraph::new(context.command.as_ref())
            .style(match context.current_action {
                ScreenAction::TypingCommand => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(vec![Span::from("Command")]),
            )
    }

    fn build_podcast_progress_bar(context: &ScreenContext) -> Gauge {
        let status = &context.player_status;

        let default_duration = DurationWrapper::default();
        let (progress, duration, percentage) = match status {
            PlayerStatus::Playing(prog, dur, perc) => (prog, dur, *perc),
            PlayerStatus::Paused(prog, dur, perc) => (prog, dur, *perc),
            PlayerStatus::Stopped => (&default_duration, &default_duration, 0),
        };

        Gauge::default()
            .block(Block::default().title("").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::LightYellow))
            .label(format!("{progress}/{duration}"))
            .percent(percentage.into())
    }

    fn build_output_layout(_context: &ScreenContext, size: &Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Max(1)].as_ref())
            .split(*size)
    }

    fn build_output_field_paragraph(context: &ScreenContext) -> Paragraph {
        let empty_string: String = "".to_string();
        assert!(context.last_command_output == OutputType::RawString(empty_string));

        let content = if let OutputType::RawString(ref s) = &context.last_command_output {
            s.clone()
        } else {
            String::default()
        };

        Paragraph::new(content)
            .style(match context.current_action {
                ScreenAction::ScrollingOutput => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Output"))
            .wrap(Wrap { trim: true })
    }

    fn build_output_field_list(context: &ScreenContext, available_width: usize) -> List {
        let last_command_output = match context.last_command_output {
            OutputType::Episodes(ref episodes) => episodes
                .iter()
                .map(move |e| build_list_item_from_episode(e, available_width))
                .collect::<Vec<ListItem>>(),
            OutputType::Podcasts(ref podcasts) => podcasts
                .iter()
                .map(move |p| build_list_item_from_podcast(p, available_width))
                .collect::<Vec<ListItem>>(),
            _ => unimplemented!(),
        };

        List::new(last_command_output)
            .style(match context.current_action {
                ScreenAction::ScrollingOutput => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Output"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightMagenta)
                    .add_modifier(Modifier::ITALIC),
            )
    }

    fn draw_main_screen<B: Backend>(&self, f: &mut Frame<B>, context: &ScreenContext) {
        let size = f.size();
        let minimal_width: u16 = 15;

        // Defining screen layout
        let chunks = MinimalisticUiDrawer::build_screen_layout(context, &size);

        let input = MinimalisticUiDrawer::build_input_field(context);
        f.render_widget(input, chunks[0]);

        let podcast_progress = MinimalisticUiDrawer::build_podcast_progress_bar(context);

        if chunks[1].width > minimal_width {
            f.render_widget(podcast_progress, chunks[1]);
        }

        let output_layout = MinimalisticUiDrawer::build_output_layout(context, &chunks[2]);

        if context.last_command_output == OutputType::RawString("".to_string()) {
            let output_paragraph = MinimalisticUiDrawer::build_output_field_paragraph(context);
            f.render_widget(output_paragraph, output_layout[0]);
        } else {
            let available_width: usize = output_layout[0].width.into();
            let output_list =
                MinimalisticUiDrawer::build_output_field_list(context, available_width);
            f.render_stateful_widget(
                output_list,
                output_layout[0],
                &mut context
                    .list_output_state
                    .as_ref()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap(),
            )
        }

        //if false {
        //    let progress_bar_height = chunks[2].height - 2;
        //    let mut progress_bar_string = String::from("");
        //    let progress_ratio = display_starting_index * usize::from(progress_bar_height)
        //        / context.last_formatted_command_output.0.len();
        //    progress_bar_string = "\n".repeat(progress_ratio);
        //    let content_ratio = displayed_output_length * usize::from(progress_bar_height)
        //        / context.last_formatted_command_output.0.len();
        //    progress_bar_string += &String::from("#".repeat(content_ratio));
        //    error!("{}", progress_bar_string);
        //
        //    let output_progress_bar = Paragraph::new(progress_bar_string)
        //        .style(Style::default().bg(Color::Gray).fg(Color::Black))
        //        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        //        .wrap(Wrap { trim: true });
        //    f.render_widget(output_progress_bar, output_layout[1]);
        //}
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
