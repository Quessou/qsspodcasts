use command_management::output::output_type::OutputType;
use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

pub struct VecSpans<'a>(pub Vec<Spans<'a>>);

impl From<&str> for VecSpans<'_> {
    fn from(s: &str) -> Self {
        VecSpans(vec![Spans::from(String::from(s))])
    }
}

impl From<&Vec<Podcast>> for VecSpans<'_> {
    fn from(podcasts: &Vec<Podcast>) -> Self {
        let mut spans = vec![];
        for podcast in podcasts.iter() {
            let p = (*podcast).clone();
            spans.extend(vec![
                Spans::from(Span::styled::<String>(
                    p.title,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Spans::from(Span::styled::<String>(
                    p.description,
                    Style::default().add_modifier(Modifier::ITALIC),
                )),
            ]);
        }
        VecSpans(spans)
    }
}

impl From<OutputType> for VecSpans<'_> {
    fn from(output: OutputType) -> Self {
        match output {
            OutputType::RawString(s) => s.as_str().into(),
            OutputType::Podcasts(p) => (&p).into(),
            _ => VecSpans(vec![]), // TODO : Implement this for PodcastEpisodes
        }
    }
}

impl From<&OutputType> for VecSpans<'_> {
    fn from(output: &OutputType) -> Self {
        match output {
            OutputType::RawString(s) => s.as_str().into(),
            OutputType::Podcasts(p) => (&p).clone().into(),
            _ => VecSpans(vec![]), // TODO : Implement this for PodcastEpisodes
        }
    }
}
