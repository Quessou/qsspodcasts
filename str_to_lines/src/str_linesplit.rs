//pub fn str_to_lines(input: &str, line_width: usize) -> Vec<String> {
//    vec![]
//}

//pub fn str_to_lines(input: &str, line_width: usize) -> Vec<String> {
//    let mut lines: Vec<String> = input
//        .trim_start()
//        .split("\n")
//        .map(String::from)
//        .collect::<_>();
//    lines = lines.iter().map(|s| String::from(s.trim_start())).collect();
//    let mut i: usize = 0;
//    while i < lines.len() {
//        let mut l = &lines[i];
//        while l.len() > line_width {
//            let last_space_index = l[..line_width + 1].rfind(" ");
//            let mut last_space_index: usize = 0;
//
//            if last_space_index == None {
//                break;
//            }
//
//            let first_line: String = String::from(&l[0..last_space_index.unwrap()]);
//            let second_line: String = String::from(&l[last_space_index.unwrap() + 1..]);
//            lines[i] = first_line;
//            lines.insert(i + 1, second_line);
//            l = &lines[i];
//        }
//        i += 1;
//    }
//
//    lines
//}

fn get_line_width(line: &Vec<&str>) -> usize {
    if line.len() == 0 {
        return 0;
    }
    let width: usize = line
        .iter()
        .fold(0, |accum, word| accum + word.chars().count());
    width + line.len() - 1
}

pub fn can_line_contain_word(line: &Vec<&str>, word_length: usize, line_width: usize) -> bool {
    let additional_space: usize = if line.len() == 0 { 0 } else { 1 };
    line_width < get_line_width(line) + word_length + additional_space
}

pub fn str_to_lines_inner(input: &str, line_width: usize) -> Vec<String> {
    let mut words = input.split_whitespace().map(|s| (s, s.chars().count()));
    let mut lines: Vec<String> = vec![];
    let mut current_line: Vec<&str> = vec![];

    while let Some((word, word_length)) = words.next() {
        if can_line_contain_word(&current_line, word_length, line_width) {
            lines.push(current_line.join(" "));
            current_line = vec![];
        }

        current_line.push(word);
    }

    if current_line.len() != 0 {
        lines.push(current_line.join(" "));
    }

    lines
}

pub fn str_to_lines(input: &str, line_width: usize) -> Vec<String> {
    let lines = input.split("\n");

    let lines = lines.fold(vec![], |mut accum, line| {
        let mut lines_split = str_to_lines_inner(line, line_width);
        accum.append(&mut lines_split);
        accum
    });

    lines
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use super::*;

    #[test_case("tutu toto", 4  => vec!["tutu", "toto"]; "Add line break to split between words")]
    #[test_case("你好吗 你好吗", 4 => vec!["你好吗", "你好吗"]; "Unicode management")]
    #[test_case("toto toto tototo", 6  => vec!["toto", "toto", "tototo"]; "Test iterative line split")]
    #[test_case("to\nto", 2  => vec!["to", "to"]; "Edge case where there's a tricky carriage return")]
    #[test_case("toto\n", 4  => vec!["toto"]; "Edge case where there's a carriage return at the end")]
    #[test_case("\ntoto", 4  => vec!["toto"]; "Edge case where there's a carriage return at the beginning")]
    #[test_case("toto toto", 5  => vec!["toto", "toto"]; "Add line break to split between words when there's not enough space to write an entire word")]
    #[test_case("toto toto\ntoto", 5  => vec!["toto", "toto", "toto"]; "Case with a line too long and a carriage return")]
    fn test_str_to_lines(input: &str, line_width: usize) -> Vec<String> {
        str_to_lines(input, line_width)
    }
}
