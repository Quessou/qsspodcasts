pub fn str_to_lines(input: &String, line_width: usize) -> Vec<String> {
    let mut lines: Vec<String> = input
        .trim_start()
        .split("\n")
        .map(String::from)
        .collect::<_>();
    lines = lines.iter().map(|s| String::from(s.trim_start())).collect();
    let mut i: usize = 0;
    while i < lines.len() {
        let mut l = &lines[i];
        while l.len() > line_width {
            let last_space_index = l[..line_width + 1].rfind(" ");
            if last_space_index == None {
                break;
            }
            let first_line: String = String::from(&l[0..last_space_index.unwrap()]);
            let second_line: String = String::from(&l[last_space_index.unwrap() + 1..]);
            lines[i] = first_line;
            lines.insert(i + 1, second_line);
            l = &lines[i];
        }
        i += 1;
    }

    //lines = lines
    //    .into_iter()
    //    .filter(|s| !s.is_empty())
    //    .collect::<Vec<String>>();

    lines
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use super::*;

    #[test_case("toto toto tototo", 6  => vec!["toto", "toto", "tototo"]; "Test iterative line split")]
    #[test_case("to\nto", 2  => vec!["to", "to"]; "Edge case where there's a tricky carriage return")]
    #[test_case("toto\n", 4  => vec!["toto", ""]; "Edge case where there's a carriage return at the end")]
    #[test_case("\ntoto", 4  => vec!["toto"]; "Edge case where there's a carriage return at the beginning")]
    #[test_case("toto toto", 4  => vec!["toto", "toto"]; "Add line break to split between words")]
    #[test_case("toto toto", 5  => vec!["toto", "toto"]; "Add line break to split between words when there's not enough space to write an entire word")]
    #[test_case("toto toto\ntoto", 5  => vec!["toto", "toto", "toto"]; "Case with a line too long and a carriage return")]
    fn test_str_to_lines(input: &str, line_width: usize) -> Vec<String> {
        str_to_lines(input, line_width)
    }
}
