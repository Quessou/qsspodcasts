use command_management::output::output_type::OutputType;
use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

use std::iter;
use str_to_lines::str_linesplit::str_to_lines;

#[derive(PartialEq, Debug)]
pub struct VecListItems<'a>(pub Vec<ListItem<'a>>);

impl From<&Vec<Podcast>> for VecListItems<'_> {
    fn from(podcasts: &Vec<Podcast>) -> Self {
        let mut items = vec![];
        for podcast in podcasts.iter() {
            items.push(ListItem::new(vec![
                Spans::from(Span::styled::<String>(
                    podcast.title.clone(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::styled::<String>(
                    podcast.description.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                )),
            ]));
        }
        VecListItems(items)
    }
}

impl From<&OutputType> for VecListItems<'_> {
    fn from(output: &OutputType) -> Self {
        match output {
            OutputType::RawString(_) => VecListItems(vec![]),
            OutputType::Podcasts(p) => p.into(),
            _ => VecListItems(vec![]), // TODO : Implement this for PodcastEpisodes
        }
    }
}

#[derive(Clone)]
pub struct ListItemFactory {
    available_width: usize,
}

impl ListItemFactory {
    pub fn new(available_width: usize) -> ListItemFactory {
        ListItemFactory { available_width }
    }

    pub fn build_from_podcast(&self, podcast: Podcast) -> ListItem {
        let vec_spans = iter::once(Spans::from(Span::styled::<String>(
            podcast.title.clone(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));

        let description_style = Style::default().add_modifier(Modifier::ITALIC);

        let lines = str_to_lines(&podcast.description, self.available_width)
            .into_iter()
            .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
        let vec_spans: Vec<Spans> = vec_spans.chain(lines).collect::<Vec<Spans>>();

        ListItem::new(vec_spans)
    }
}

pub fn build_list_item_from_podcast(podcast: &Podcast, available_width: usize) -> ListItem {
    let vec_spans = iter::once(Spans::from(Span::styled::<String>(
        podcast.title.clone(),
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )));

    let description_style = Style::default().add_modifier(Modifier::ITALIC);

    let lines = str_to_lines(&podcast.description, available_width)
        .into_iter()
        .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
    let vec_spans: Vec<Spans> = vec_spans.chain(lines).collect::<Vec<Spans>>();

    ListItem::new(vec_spans)
}
