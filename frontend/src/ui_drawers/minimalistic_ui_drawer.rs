use command_management::output::output_type::OutputType;
use log::debug;
use podcast_management::data_objects::hashable::Hashable;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_status::PlayerStatus;
use std::borrow::{Borrow, Cow};
use std::iter;
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

use str_to_lines::str_linesplit::str_to_lines;

use super::ui_drawer;

pub struct MinimalisticUiDrawer<'a> {
    pub cached_output: Cow<'a, Vec<ListItem<'a>>>,
}

impl MinimalisticUiDrawer<'_> {
    pub fn new() -> Self {
        MinimalisticUiDrawer {
            cached_output: Cow::Owned(vec![]),
        }
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

    fn is_output_cache_invalidated(context: &ScreenContext, available_width: usize) -> bool {
        context.must_invalidate_cache.get()
            || context.previous_output_pane_available_width.get().unwrap() != available_width
    }

    fn build_output_field_list(&mut self, context: &ScreenContext, available_width: usize) -> List {
        if MinimalisticUiDrawer::is_output_cache_invalidated(context, available_width) {
            match &context.last_command_output {
                OutputType::Episodes(episodes) => {
                    let output = episodes
                        .iter()
                        .map(move |e| {
                            let vec_spans = iter::once(Spans::from(Span::styled(
                                e.title.clone(),
                                Style::default().bg(Color::LightGreen).fg(Color::Red),
                            )));

                            let metadata_display = iter::once(Spans::from(vec![
                                Span::from("["),
                                Span::styled(
                                    e.hash(),
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(Color::LightGreen),
                                ),
                                Span::from("]"),
                                Span::from("   "),
                                Span::styled(
                                    "Date:",
                                    Style::default().add_modifier(Modifier::ITALIC),
                                ),
                                Span::from(" "),
                                Span::styled(
                                    format!("{}", e.pub_date.format("%d/%m/%Y")),
                                    Style::default()
                                        .bg(Color::Black)
                                        .add_modifier(Modifier::ITALIC),
                                ),
                            ]));
                            let vec_spans = vec_spans.chain(metadata_display);

                            let description_style = Style::default().add_modifier(Modifier::ITALIC);

                            let description = &e.description;
                            let lines = str_to_lines(description, available_width)
                                .into_iter()
                                .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
                            let vec_spans: Vec<Spans> =
                                vec_spans.chain(lines).collect::<Vec<Spans>>();

                            ListItem::new(vec_spans)
                        })
                        .collect::<Vec<ListItem>>();
                    self.cached_output = Cow::Owned(output);
                }
                OutputType::Podcasts(podcasts) => {
                    let output = podcasts
                        .iter()
                        .map(move |p| {
                            let vec_spans = iter::once(Spans::from(Span::styled::<String>(
                                p.title.clone(),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            )));

                            let hash_display = iter::once(Spans::from(vec![
                                Span::from("["),
                                Span::styled(
                                    p.hash(),
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .fg(Color::LightGreen),
                                ),
                                Span::from("]"),
                            ]));
                            let vec_spans = vec_spans.chain(hash_display);

                            let description_style = Style::default().add_modifier(Modifier::ITALIC);

                            let lines = str_to_lines(&p.description, available_width)
                                .into_iter()
                                .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
                            let vec_spans = vec_spans.chain(lines).collect::<Vec<Spans>>();

                            ListItem::new(vec_spans)
                        })
                        .collect::<Vec<ListItem>>();
                    self.cached_output = Cow::Owned(output);
                }
                _ => unimplemented!(),
            };
            context.must_invalidate_cache.set(false);
        }

        let title = "Output";

        List::new(&self.cached_output[..])
            .style(match context.current_action {
                ScreenAction::ScrollingOutput => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(
                Style::default()
                    .bg(Color::LightMagenta)
                    .add_modifier(Modifier::ITALIC),
            )
    }

    fn draw_main_screen<B: Backend>(&mut self, f: &mut Frame<B>, context: &ScreenContext) {
        let size = f.size();
        const MINIMAL_WIDTH: u16 = 15;

        // Defining screen layout
        let chunks = MinimalisticUiDrawer::build_screen_layout(context, &size);

        let input = MinimalisticUiDrawer::build_input_field(context);
        f.render_widget(input, chunks[0]);

        let podcast_progress = MinimalisticUiDrawer::build_podcast_progress_bar(context);

        if chunks[1].width > MINIMAL_WIDTH {
            f.render_widget(podcast_progress, chunks[1]);
        }

        let output_layout = MinimalisticUiDrawer::build_output_layout(context, &chunks[2]);

        if context.last_command_output == OutputType::RawString("".to_string()) {
            let output_paragraph = MinimalisticUiDrawer::build_output_field_paragraph(context);
            f.render_widget(output_paragraph, output_layout[0]);
        } else {
            let available_width: usize = output_layout[0].width.into();
            let output_list = self.build_output_field_list(context, available_width);
            f.render_stateful_widget(
                output_list,
                output_layout[0],
                &mut context
                    .list_output_state
                    .as_ref()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap(),
            );

            context
                .previous_output_pane_available_width
                .set(Some(available_width));
        }

        let list_state = context.list_output_state.borrow();
        if list_state.is_some() {
            let progress_bar_height = chunks[2].height - 2;
            let selected_index = list_state
                .as_ref()
                .unwrap()
                .try_borrow()
                .unwrap()
                .selected()
                .unwrap_or_default();
            let list_length = context.get_output_list_length().unwrap();
            let progress_ratio = selected_index * usize::from(progress_bar_height) / list_length;
            let mut progress_bar_string = "\n".repeat(progress_ratio);
            progress_bar_string.push('#');

            let output_progress_bar = Paragraph::new(progress_bar_string)
                .style(Style::default().bg(Color::Gray).fg(Color::Black))
                .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
                .wrap(Wrap { trim: true });
            f.render_widget(output_progress_bar, output_layout[1]);
        }
    }
}

impl ui_drawer::UiDrawer for MinimalisticUiDrawer<'_> {
    fn draw_ui<B: Backend>(&mut self, f: &mut Frame<B>, context: &ScreenContext) {
        debug!("Starting drawing next frame");
        match context.current_action {
            ScreenAction::ScrollingLogs => self.draw_log_screen(f, context),
            _ => self.draw_main_screen(f, context),
        }
        debug!("Finished drawing next frame");
    }
}
impl Default for MinimalisticUiDrawer<'_> {
    fn default() -> Self {
        Self::new()
    }
}
