use super::output_size_computationer;
use tui::text::Spans;

pub fn output_overflow(
    complete_output: &Vec<Spans>,
    output_pane_width: usize,
    output_pane_height: usize,
) -> bool {
    let output_height =
        output_size_computationer::get_output_height(&complete_output, output_pane_width);
    output_height > output_pane_height
}
