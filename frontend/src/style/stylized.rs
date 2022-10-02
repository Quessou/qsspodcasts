use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

pub type StylizedContent<'a> = Vec<Spans<'a>>;

pub trait Stylized {
    fn to_stylized(&self) -> StylizedContent;
}

impl Stylized for String {
    fn to_stylized(&self) -> StylizedContent {
        vec![Spans::from(self.as_ref())]
    }
}

impl Stylized for Podcast {
    fn to_stylized(&self) -> StylizedContent {
        vec![
            Spans::from(Span::styled::<&str>(
                self.title.as_ref(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Spans::from(Span::styled::<&str>(
                self.description.as_ref(),
                Style::default().add_modifier(Modifier::ITALIC),
            )),
        ]
    }
}
