use super::vec_spans::VecSpans;
use super::{output_size_computationer, vec_spans};
use command_management::output::output_type::OutputType;
use tui::text::Spans;

pub fn spans_output_overflow(
    complete_output: &Vec<Spans>,
    output_pane_width: usize,
    output_pane_height: usize,
) -> bool {
    let output_height =
        output_size_computationer::get_output_height(&complete_output, output_pane_width);
    output_height > output_pane_height
}

pub fn output_overflow<T>(output: &T, output_pane_width: usize, output_pane_height: usize) -> bool
where
    for<'a> vec_spans::VecSpans<'a>: From<&'a T>,
{
    let co: VecSpans = output.clone().into();
    spans_output_overflow(&co.0, output_pane_width, output_pane_height)
}
