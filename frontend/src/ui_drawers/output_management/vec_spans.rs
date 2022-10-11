use command_management::output::output_type::OutputType;
use podcast_management::data_objects::podcast::Podcast;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

use str_to_lines::str_linesplit::str_to_lines;
#[derive(PartialEq, Debug)]
pub struct VecSpans<'a>(pub Vec<Spans<'a>>);

/****
 * Ligne de 4 de largeur
 *
 * VecSpans => [ Spans [ Span [ "toto toto" ] ] ]
 * On veut :
 * VecSpans => [ Spans [ Span ["toto"] ], Spans [ Span [ "toto" ] ] ]
 *
 * On veut UN *SPANS* (et pas un SPAN) par ligne.
 * Pourquoi un SPANS et pas un SPAN ?
 * Parce que l'on peut vouloir sur la même ligne des choses qui ont un style différent
*/

//impl VecSpans<'_> {
//    pub fn to_lines(self, line_width: usize) -> Self {
//        let mut lines: Vec<Spans> = vec![];
//        for spans in self.0.iter() {
//            if spans.width() < line_width && spans.0.iter().all(|s| !s.content.contains("\n")) {
//                lines.push(spans.to_owned());
//            } else {
//                let mut new_vec_spans: Vec<Spans> = vec![];
//                let mut last_spans: Spans = Spans::default();
//                for span in spans.0.iter() {
//                    let span_to_lines = str_to_lines(&(*span.content), line_width);
//                    let mut tmp: Vec<Spans> = span_to_lines
//                        .into_iter()
//                        .map(|s| Spans(vec![Span::styled(s, span.style)]))
//                        .collect::<Vec<Spans>>();
//                    if last_spans != Spans::default()
//                        && last_spans.width() + tmp[0].width() <= line_width
//                    {
//                        // TODO
//                        let mut toto = tmp[0].0.clone();
//                        new_vec_spans.last().unwrap().0.append(toto.clone());
//                        tmp = tmp[1..].to_vec();
//                    }
//
//                    new_vec_spans.append(&mut tmp);
//                    last_spans = new_vec_spans.last().unwrap().clone();
//                }
//
//                lines.append(&mut new_vec_spans);
//            }
//        }
//
//        VecSpans(lines)
//    }
//}
//
//impl From<&str> for VecSpans<'_> {
//    fn from(s: &str) -> Self {
//        VecSpans(vec![Spans::from(String::from(s))])
//    }
//}
//
//impl From<&Vec<Podcast>> for VecSpans<'_> {
//    fn from(podcasts: &Vec<Podcast>) -> Self {
//        let mut spans = vec![];
//        for podcast in podcasts.iter() {
//            let p = (*podcast).clone();
//            spans.extend(vec![
//                Spans::from(Span::styled::<String>(
//                    p.title,
//                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
//                )),
//                Spans::from(Span::styled::<String>(
//                    p.description,
//                    Style::default().add_modifier(Modifier::ITALIC),
//                )),
//            ]);
//        }
//        VecSpans(spans)
//    }
//}
//
//impl From<OutputType> for VecSpans<'_> {
//    fn from(output: OutputType) -> Self {
//        match output {
//            OutputType::RawString(s) => s.as_str().into(),
//            OutputType::Podcasts(p) => (&p).into(),
//            _ => VecSpans(vec![]), // TODO : Implement this for PodcastEpisodes
//        }
//    }
//}
//
//impl From<&OutputType> for VecSpans<'_> {
//    fn from(output: &OutputType) -> Self {
//        match output {
//            OutputType::RawString(s) => s.as_str().into(),
//            OutputType::Podcasts(p) => (&p).clone().into(),
//            _ => VecSpans(vec![]), // TODO : Implement this for PodcastEpisodes
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    use test_case::test_case;
//
//    #[test_case(VecSpans(vec![ Spans(vec![ Span::raw("toto toto") ]) ]), 4 => VecSpans(vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto") ]) ]); "Very simple split case")]
//    #[test_case(VecSpans(vec![ Spans(vec![ Span::raw("toto\ntoto") ]) ]), 4 => VecSpans(vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto") ]) ]); "Very simple split case with line break")]
//    #[test_case(VecSpans(vec![ Spans(vec![ Span::raw("toto\ntoto"), Span::styled("tata", Style::default().bg(Color::Yellow)) ]) ]), 10 => VecSpans(vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto"), Span::styled("tata", Style::default().bg(Color::Yellow)) ]) ]); "Split case with style transition on the same line")]
//    fn test_vecspans_to_lines(vec_spans: VecSpans, line_width: usize) -> VecSpans {
//        vec_spans.to_lines(line_width)
//    }
//}
//
