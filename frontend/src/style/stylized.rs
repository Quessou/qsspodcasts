use command_management::output::output_type::OutputType;
use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

// TODO : Remove this file

// TODO : Move this
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
    fn from(output_type: OutputType) -> Self {
        let dummy = String::from("");
        let complete_output: VecSpans = match &output_type {
            OutputType::RawString(s) => s.as_str().into(),
            OutputType::Podcasts(podcasts) => podcasts.into(),
            _ => dummy.as_str().into(),
        };
        complete_output
    }
}

//impl From<OutputType> for VecSpans<'_> {
//    fn from(output: OutputType) -> Self {
//        match output {
//            OutputType::RawString(s) => s.into(),
//            OutputType::Podcasts(p) => p.into(),
//            _ => VecSpans(vec![]), // TODO : Implement this for PodcastEpisodes
//        }
//    }
//}

//pub trait Stylized {
//    fn to_stylized(self) -> StylizedContent;
//}
//
//impl Stylized for String {
//    fn to_stylized(self) -> StylizedContent {
//        vec![Spans::from(self)]
//    }
//}
//
//impl Stylized for Podcast {
//    fn to_stylized(&self) -> StylizedContent {
//        vec![
//            Spans::from(Span::styled::<&str>(
//                self.title.as_ref(),
//                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
//            )),
//            Spans::from(Span::styled::<&str>(
//                self.description.as_ref(),
//                Style::default().add_modifier(Modifier::ITALIC),
//            )),
//        ]
//    }
//}
//
