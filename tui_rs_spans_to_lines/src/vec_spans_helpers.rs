use tui::style::{Color, Style};
use tui::text::{Span, Spans};

pub fn to_lines(spans: Vec<Spans>, line_length: usize) -> Vec<Spans> {
    let mut lines: Vec<Spans> = vec![];

    lines
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case(vec![ Spans(vec![ Span::raw("toto toto") ]) ], 4 => vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto") ]) ]; "Very simple split case")]
    #[test_case(vec![ Spans(vec![ Span::raw("toto\ntoto") ]) ], 4 => vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto") ]) ]; "Very simple split case with line break")]
    #[test_case(vec![ Spans(vec![ Span::raw("toto\ntoto"), Span::styled("tata", Style::default().bg(Color::Yellow)) ]) ], 10 => vec![ Spans(vec![ Span::raw("toto") ]), Spans(vec![ Span::raw("toto"), Span::styled("tata", Style::default().bg(Color::Yellow)) ]) ]; "Split case with style transition on the same line")]
    fn test_vecspans_to_lines(vec_spans: Vec<Spans>, line_width: usize) -> Vec<Spans> {
        to_lines(vec_spans, line_width)
    }
}
