use super::output_size_computationer;
use tui::text::Spans;

pub fn filter_displayed_output<'a>(
    complete_output: Vec<Spans<'a>>,
    output_pane_width: usize,
    output_pane_height: usize,
) -> Vec<Spans<'a>> {
    if complete_output.len() == 0 {
        return vec![];
    }

    let mut used_lines: usize = 0;
    let mut displayed_output = vec![];
    let mut i: usize = 0;
    while used_lines <= output_pane_height && i < complete_output.len() {
        used_lines +=
            output_size_computationer::get_spans_height(&complete_output[i], output_pane_width);
        displayed_output.push(complete_output[i].clone());
        i += 1;
    }
    //if used_lines > output_pane_height {
    //    displayed_output.pop();
    //}

    displayed_output
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use super::*;

    #[test_case(vec![Spans::from("t")], 1, 1 => 1; "Small unique span")]
    //#[test_case(vec![Spans::from("tt"), Spans::from("tt")], 1, 3 => 1; "spans with line breaks that does not fit in the output")]
    #[test_case(vec![Spans::from("tt"), Spans::from("tt")], 1, 4 => 2; "spans with line breaks that does fit in the output")]
    fn test_output_filter(spans: Vec<Spans>, output_width: usize, output_height: usize) -> usize {
        let displayed_output = filter_displayed_output(spans, output_width, output_height);
        displayed_output.len()
    }
}
