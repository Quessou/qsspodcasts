use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

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
        //VecSpans(vec![
        //    Spans::from(Span::styled::<&str>(
        //        p.title.as_ref(),
        //        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        //    )),
        //    Spans::from(Span::styled::<&str>(
        //        p.description.as_ref(),
        //        Style::default().add_modifier(Modifier::ITALIC),
        //    )),
        //])
    }
}

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
