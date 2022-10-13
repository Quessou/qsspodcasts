// TODO : Rename me

use podcast_management::data_objects::hashable::Hashable;
use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::ListItem;

use std::iter;
use str_to_lines::str_linesplit::str_to_lines;

#[derive(PartialEq, Debug)]
pub struct VecListItems<'a>(pub Vec<ListItem<'a>>);

pub fn build_list_item_from_podcast(podcast: &Podcast, available_width: usize) -> ListItem {
    let vec_spans = iter::once(Spans::from(Span::styled::<String>(
        podcast.title.clone(),
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )));

    let hash_display = iter::once(Spans::from(vec![
        Span::from("["),
        Span::styled(
            podcast.hash(),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightGreen),
        ),
        Span::from("]"),
    ]));
    let vec_spans = vec_spans.chain(hash_display);

    let description_style = Style::default().add_modifier(Modifier::ITALIC);

    let lines = str_to_lines(&podcast.description, available_width)
        .into_iter()
        .map(|s| Spans::from(vec![Span::styled(s, description_style)]));
    let vec_spans: Vec<Spans> = vec_spans.chain(lines).collect::<Vec<Spans>>();

    ListItem::new(vec_spans)
}
