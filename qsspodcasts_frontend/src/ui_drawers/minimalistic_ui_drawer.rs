use command_management::output::output_type::OutputType;
use log::debug;
use podcast_management::data_objects::hashable::Hashable;
use podcast_management::data_objects::podcast_state::PodcastState;
use podcast_player::duration_wrapper::DurationWrapper;
use podcast_player::player_status::PlayerStatus;
use std::borrow::{Borrow, Cow};

use std::iter;
use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::style::Modifier;
use tui::text::{Line, Span};
use tui::Frame;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
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
            .map(|s| ListItem::new(Line::from(vec![Span::raw(s)])))
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
                    Constraint::Length(6),
                ]
                .as_ref(),
            )
            .split(*size)
            .to_vec()
    }

    fn build_input_field(context: &ScreenContext) -> Paragraph {
        Paragraph::new(context.autocompletion_context.get_displayed_input())
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
            PlayerStatus::Stopped(o) => match o {
                Some((prog, dur, perc)) => (prog, dur, *perc),
                None => (&default_duration, &default_duration, 0),
            },
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
            .to_vec()
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

    /// TODO : Clean this awful shit
    fn build_output_field_list(&mut self, context: &ScreenContext, available_width: usize) -> List {
        if MinimalisticUiDrawer::is_output_cache_invalidated(context, available_width) {
            match &context.last_command_output {
                OutputType::Episodes(episodes) => {
                    let output = episodes
                        .iter()
                        .map(move |e| {
                            let vec_spans = iter::once(Line::from(Span::styled(
                                e.title.clone(),
                                Style::default().bg(Color::LightGreen).fg(Color::Red),
                            )));

                            let mut metadata_display = vec![
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
                            ];
                            if let Some(PodcastState::Finished) =
                                context.podcasts_state_cache.get_podcast_state(&e.hash())
                            {
                                metadata_display.append(&mut vec![
                                    Span::from("    "),
                                    Span::styled(
                                        "[FINISHED]",
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::Red),
                                    ),
                                ]);
                            }
                            let metadata_display = iter::once(Line::from(metadata_display));
                            let vec_spans = vec_spans.chain(metadata_display);

                            let description_style = Style::default().add_modifier(Modifier::ITALIC);

                            let description = &e.description;
                            let lines = str_to_lines(description, available_width)
                                .into_iter()
                                .map(|s| Line::from(vec![Span::styled(s, description_style)]));
                            let vec_spans: Vec<Line> =
                                vec_spans.chain(lines).collect::<Vec<Line>>();

                            ListItem::new(vec_spans)
                        })
                        .collect::<Vec<ListItem>>();
                    self.cached_output = Cow::Owned(output);
                }
                OutputType::Podcasts(podcasts) => {
                    let output = podcasts
                        .iter()
                        .map(move |p| {
                            let vec_spans = iter::once(Line::from(Span::styled::<String>(
                                p.title.clone(),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            )));

                            let hash_display = iter::once(Line::from(vec![
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
                                .map(|s| Line::from(vec![Span::styled(s, description_style)]));
                            let vec_spans = vec_spans.chain(lines).collect::<Vec<Line>>();

                            ListItem::new(vec_spans)
                        })
                        .collect::<Vec<ListItem>>();
                    self.cached_output = Cow::Owned(output);
                }
                OutputType::CommandHelps(commands) => {
                    let output = commands
                        .iter()
                        .map(move |c| {
                            let vec_spans = vec![
                                Line::from(Span::styled(
                                    "NAME",
                                    Style::default().add_modifier(Modifier::BOLD),
                                )),
                                Line::from(vec![
                                    Span::raw("    "),
                                    Span::styled(
                                        c.command_name.to_owned(),
                                        Style::default()
                                            .fg(Color::Red)
                                            .add_modifier(Modifier::ITALIC),
                                    ),
                                ]),
                            ]
                            .into_iter();

                            let sample = vec![
                                Line::from(vec![Span::styled(
                                    "SAMPLE",
                                    Style::default().add_modifier(Modifier::BOLD),
                                )]),
                                Line::from(vec![Span::raw("    "), Span::raw(c.sample.to_owned())]),
                            ];
                            let vec_spans = vec_spans.chain(sample);

                            let lines = vec![Line::from(Span::styled(
                                "DESCRIPTION",
                                Style::default().add_modifier(Modifier::BOLD),
                            ))]
                            .into_iter();
                            let lines = lines.chain(
                                str_to_lines(c.description, available_width)
                                    .into_iter()
                                    .map(|s| Line::from(vec![Span::raw("   "), Span::raw(s)])),
                            );

                            let vec_spans = vec_spans.chain(lines).collect::<Vec<Line>>();
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
            .highlight_style(match context.current_action {
                ScreenAction::ScrollingOutput => Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::ITALIC),
                _ => Style::default(),
            })
    }

    fn build_notifications_field(context: &ScreenContext) -> Paragraph {
        let notifications = context
            .message_notifications_buffer
            .iter()
            .rev()
            .map(|s| Line::from(s.as_ref()));

        let empty_lines_count: i16 =
            std::cmp::max(0, 4 - (context.message_notifications_buffer.len() as i16));
        let empty_spaces =
            iter::repeat(Line::from(" ")).take(empty_lines_count.try_into().unwrap());

        let notifications = empty_spaces.chain(notifications);
        let notifications: Vec<Line> = notifications.collect();

        Paragraph::new(notifications)
            .style(Style::default().fg(Color::LightBlue))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true })
    }

    fn build_modal_window(
        &self,
        screen_size: Rect,
        percentage_height: u16,
        percentage_width: u16,
    ) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percentage_height) / 2),
                    Constraint::Percentage(percentage_height),
                    Constraint::Percentage((100 - percentage_height) / 2),
                ]
                .as_ref(),
            )
            .split(screen_size);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percentage_width) / 2),
                    Constraint::Percentage(percentage_width),
                    Constraint::Percentage((100 - percentage_width) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    /// .
    ///
    /// # TODO
    /// - Make this function more abstract to allow to use in more contexts ?
    fn build_interactable_modal_list(&self, ctxt: &ScreenContext) -> List {
        let modal_actions = ctxt
            .interactable_modal_context
            .modal_actions
            .as_ref()
            .unwrap();
        List::new(
            modal_actions
                .iter()
                .map(|a| ListItem::new(a.action.clone())) // Fuck this copy :(
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::ITALIC),
        )
    }

    /// God this function is bad.
    fn build_readonly_modal_list(&self, ctxt: &ScreenContext) -> List {
        let empty_vec = vec![];
        let read_only_content = ctxt
            .read_only_modal_context
            .content
            .as_ref()
            .unwrap_or(&empty_vec);
        if read_only_content.is_empty() {
            return List::new(vec![]);
        }

        let mut modifiers: Vec<Style> = read_only_content
            .iter()
            .map(|s| {
                let style = Style::default();
                if !s.starts_with(' ') {
                    style.add_modifier(Modifier::BOLD)
                } else {
                    style.add_modifier(Modifier::ITALIC)
                }
            })
            .collect();
        let last_element_index = modifiers.len() - 1;
        let last_style = modifiers
            .remove(last_element_index)
            .add_modifier(Modifier::ITALIC | Modifier::SLOW_BLINK);
        modifiers.push(last_style);

        let items = read_only_content
            .iter()
            .enumerate()
            .map(|(i, s)| ListItem::new(*s).style(modifiers[i]))
            .collect::<Vec<_>>();
        List::new(items).block(Block::default().borders(Borders::ALL))
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
            'display_progress_bar: {
                let progress_bar_height = chunks[2].height - 2;
                let selected_index = list_state
                    .as_ref()
                    .unwrap()
                    .try_borrow()
                    .unwrap()
                    .selected()
                    .unwrap_or_default();
                let list_length = context.get_output_list_length().unwrap();
                if list_length == 0 {
                    break 'display_progress_bar;
                }
                let progress_ratio =
                    selected_index * usize::from(progress_bar_height) / list_length;
                let mut progress_bar_string = "\n".repeat(progress_ratio);
                progress_bar_string.push('#');

                let output_progress_bar = Paragraph::new(progress_bar_string)
                    .style(Style::default().bg(Color::Gray).fg(Color::Black))
                    .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
                    .wrap(Wrap { trim: true });
                f.render_widget(output_progress_bar, output_layout[1]);
            }
        }

        let notifications_layout = MinimalisticUiDrawer::build_notifications_field(context);
        f.render_widget(notifications_layout, chunks[3]);

        if context.current_action == ScreenAction::ScrollingModalWindow {
            let block = Block::default().borders(Borders::ALL);
            let modal_window = self.build_modal_window(size, 60, 20);
            let modal_list = self.build_interactable_modal_list(context);
            f.render_widget(Clear, modal_window); //this clears out the background
            f.render_widget(block, modal_window);
            f.render_stateful_widget(
                modal_list,
                modal_window,
                &mut context
                    .interactable_modal_context
                    .modal_actions_list_state
                    .as_ref()
                    .unwrap()
                    .try_borrow_mut()
                    .unwrap(),
            )
        } else if context.current_action == ScreenAction::ShowingReadOnlyModalWindow {
            let block = Block::default().borders(Borders::ALL);
            let modal_window = self.build_modal_window(size, 60, 60);
            let modal_list = self.build_readonly_modal_list(context);
            f.render_widget(Clear, modal_window);
            f.render_widget(block, modal_window);
            f.render_widget(modal_list, modal_window);
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
