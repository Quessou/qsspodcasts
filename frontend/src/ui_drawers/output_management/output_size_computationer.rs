use tui::text::Spans;

//use crate::style::stylized::Stylized;
//
///// Returns the number of line required to display the Stylized given in parameter
//pub fn get_output_stylized_height(s: impl Stylized, output_available_width: usize) -> usize {
//    let stylized_output = s.to_stylized();
//    get_output_height(&stylized_output, output_available_width)
//}

pub fn get_output_height(spans_vec: &Vec<Spans>, output_available_width: usize) -> usize {
    let mut line_count: usize = 0;
    for spans in spans_vec {
        line_count += get_spans_height(&spans, output_available_width);
    }

    line_count
}

pub fn get_spans_height(spans: &Spans, output_available_width: usize) -> usize {
    let mut line_count: usize = 0;
    let mut field_width: usize = 0;
    for span in spans.0.iter() {
        field_width += span.content.len();
    }
    line_count += field_width / output_available_width;
    if (field_width % output_available_width) != 0 {
        line_count += 1;
    }
    line_count
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use podcast_management::data_objects::podcast::Podcast;
//    use test_case::test_case;
//
//    #[test_case(String::from("t"), 10  => 1; "Nominal basic case")]
//    #[test_case(String::from("toto"), 2  => 2; "Testing line splitting case")]
//    #[test_case(String::from("totoo"), 2  => 3; "Testing with line being incomplete")]
//    fn test_compute_string(s: String, output_width: usize) -> usize {
//        get_output_stylized_height(s, output_width)
//    }
//
//    /// TODO : update these test cases when output of podcasts will be finalized
//    #[test_case(Podcast::new("a", "", "aaa", None, None, None, vec![]), 10  => 2; "Nominal basic case")]
//    #[test_case(Podcast::new("a", "", "aaa", None, None, None, vec![]), 2  => 3; "With line break in the description")]
//    fn test_compute_podcast(p: Podcast, output_width: usize) -> usize {
//        get_output_stylized_height(p, output_width)
//    }
//}
