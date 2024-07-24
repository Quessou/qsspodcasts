use crate::AutocompletionResponse;
use command_management::autocompletion::autocompletion_command_data::AutocompletionCommandData;
use command_management::autocompletion::command_parameter_type::CommandParameterType;

mod inner {
    pub fn extract_completed_command_part(command: &str) -> String {
        let last_space = command.rfind(' ');
        match last_space {
            Some(index) => command[0..index + 1].to_owned(),
            None => "".to_owned(),
        }
    }

    pub fn extract_to_be_completed(command: &str) -> String {
        if command.ends_with(" ") {
            return "".to_owned();
        }
        deduplicate_spaces(command)
            .split_whitespace()
            .last()
            .unwrap_or("")
            .to_owned()
    }

    pub fn deduplicate_spaces(line: &str) -> String {
        let mut prev: char = 0 as char;
        let mut output = line.to_owned();
        output.retain(|ch| {
            let result = ch != ' ' || prev != ' ';
            prev = ch;
            result
        });
        output
    }

    pub fn is_command_typed(command: &str) -> bool {
        let first_non_whitespace_index = command.find(|c: char| !c.is_whitespace());
        if first_non_whitespace_index.is_none() {
            return false;
        }
        let first_non_whitespace_index = first_non_whitespace_index.unwrap();
        command[first_non_whitespace_index..].contains(|c: char| c.is_whitespace())
    }
}

pub struct Autocompleter {
    commands: Vec<AutocompletionCommandData>,
    hashes: Vec<String>,
}

impl Autocompleter {
    pub fn new(commands: Vec<AutocompletionCommandData>) -> Autocompleter {
        Autocompleter {
            commands,
            hashes: vec![],
        }
    }

    pub fn set_hashes(&mut self, hashes: Vec<String>) {
        self.hashes = hashes;
    }

    pub fn autocomplete_command(&self, to_be_completed: &str) -> Vec<String> {
        assert!(!to_be_completed.contains(' '));
        let all_commands = self.commands.iter().map(|c| c.command.to_string());

        if to_be_completed.is_empty() {
            return all_commands.collect();
        }
        all_commands
            .filter(|c| c.starts_with(to_be_completed))
            .collect()
    }

    /// Input :
    ///     - A hash to complete
    pub fn autocomplete_hash(&self, to_be_completed: &str) -> Vec<String> {
        assert!(!to_be_completed.contains(' '));
        if to_be_completed.is_empty() {
            return self.hashes.clone();
        }
        self.hashes
            .iter()
            .filter(|h| h.starts_with(to_be_completed))
            .cloned()
            .collect()
    }

    pub fn autocomplete(&self, line_to_be_autocompleted: &str) -> AutocompletionResponse {
        let completed_command_part =
            inner::extract_completed_command_part(line_to_be_autocompleted);
        let to_be_completed = inner::extract_to_be_completed(line_to_be_autocompleted);

        let autocompletion_options = if !inner::is_command_typed(line_to_be_autocompleted) {
            let mut possible_outcomes = self.autocomplete_command(&to_be_completed);
            // TODO: Optimize to remove the calls to insert_str
            possible_outcomes
                .iter_mut()
                .for_each(|hash| hash.insert_str(0, &completed_command_part));
            possible_outcomes
        } else {
            let command_with_dedup_spaces = inner::deduplicate_spaces(line_to_be_autocompleted);
            let typed_command = command_with_dedup_spaces.trim().split(' ').next().unwrap();
            let command_completion_data = self
                .commands
                .iter()
                .find(|c| c.command.to_string() == typed_command);

            if let Some(parameter_type) = command_completion_data {
                let parameter_type = &parameter_type.parameter_type;
                if parameter_type.is_none() {
                    return AutocompletionResponse::default();
                }
                match command_completion_data
                    .unwrap()
                    .parameter_type
                    .as_ref()
                    .unwrap()
                {
                    CommandParameterType::Hash => {
                        let mut possible_outcomes = self.autocomplete_hash(&to_be_completed);
                        possible_outcomes
                            .iter_mut()
                            .for_each(|hash| hash.insert_str(0, &completed_command_part));
                        possible_outcomes
                    }
                    CommandParameterType::CommandName => {
                        // TODO: Here we have to handle something for "help" command
                        let mut possible_outcomes = self.autocomplete_command(&to_be_completed);
                        possible_outcomes
                            .iter_mut()
                            .for_each(|command| command.insert_str(0, &completed_command_part));
                        possible_outcomes
                    }
                    _ => unreachable!(), // TODO : We may have to do something a bit smarter here
                                         // since someone can troll and just write shit
                                         // This crashes when doing autocompletion on "advance" for
                                         // instance
                }
            } else {
                vec![]
            }
        };

        AutocompletionResponse {
            autocompletion_options,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use command_management::commands::command_enum::Command;

    #[test]
    fn test_autocomplete_command() {
        let autocompleter = Autocompleter::new(vec![
            AutocompletionCommandData::new(Command::Exit, None),
            AutocompletionCommandData::new(Command::ListPodcasts, None),
            AutocompletionCommandData::new(Command::ListEpisodes(None), None),
        ]);
        let mut command_to_be_completed = String::from("ex");
        let autocomplete_choices = autocompleter.autocomplete_command(&command_to_be_completed);
        assert_eq!(autocomplete_choices.len(), 1);
        assert_eq!(autocomplete_choices[0], "exit");

        command_to_be_completed = String::from("lis");
        let autocomplete_choices = autocompleter.autocomplete_command(&command_to_be_completed);
        assert_eq!(autocomplete_choices.len(), 2);
    }

    #[test]
    fn test_autocomplete_hash() {
        let mut autocompleter = Autocompleter::new(vec![]);
        autocompleter.set_hashes(vec![
            "42ff03".to_owned(),
            "400001".to_owned(),
            "813829".to_owned(),
        ]);
        let hash_to_be_completed = String::from("81");
        let autocomplete_choices = autocompleter.autocomplete_hash(&hash_to_be_completed);
        assert_eq!(autocomplete_choices.len(), 1);
        assert_eq!(autocomplete_choices[0], "813829");

        let hash_to_be_completed = String::from("4");
        let autocomplete_choices = autocompleter.autocomplete_hash(&hash_to_be_completed);
        assert_eq!(autocomplete_choices.len(), 2);
    }

    #[test_case("" => "".to_owned(); "Empty string")]
    #[test_case(" " => " ".to_owned(); "One space in string")]
    #[test_case("    " => " ".to_owned(); "Several spaces in string")]
    #[test_case("toto tata" => "toto tata".to_owned(); "Nothing to do")]
    #[test_case("toto  tata" => "toto tata".to_owned(); "One space to remove")]
    #[test_case("toto tata  " => "toto tata ".to_owned(); "Space to remove at the end")]
    #[test_case("  toto tata" => " toto tata".to_owned(); "Space to remove at beginning")]
    #[test_case("  toto tata  " => " toto tata ".to_owned(); "Space to remove at beginning and at the end")]
    #[test_case("  toto  tata  " => " toto tata ".to_owned(); "Space to remove everywhere lol")]
    fn test_deduplicate_spaces(s: &str) -> String {
        super::inner::deduplicate_spaces(s)
    }

    #[test_case("toto tata" => "tata".to_owned(); "Return last word")]
    #[test_case("toto tata " => "".to_owned(); "Return empty string if line ends with a white space")]
    #[test_case("toto tata   " => "".to_owned(); "Return empty string if line ends with several white spaces")]
    #[test_case("toto   tata" => "tata".to_owned(); "Return last word if line has several white spaces bulked together")]
    #[test_case("toto" => "toto".to_owned(); "Return only word if there's no space")]
    #[test_case("" => "".to_owned(); "Return empty string if empty")]
    #[test_case(" " => "".to_owned(); "Return empty string if only one space")]
    #[test_case("    " => "".to_owned(); "Return empty string if only several spaces")]
    fn test_extract_to_be_completed(s: &str) -> String {
        super::inner::extract_to_be_completed(s)
    }

    #[test_case("toto tata" => "toto ".to_owned(); "Basic case")]
    #[test_case("toto" => "".to_owned(); "No space (thus the completed command is empty)")]
    #[test_case("  toto" => "  ".to_owned(); "Spaces and a word")]
    #[test_case("" => "".to_owned(); "Empty string")]
    #[test_case("  " => "  ".to_owned(); "Whitespaces only string")]
    #[test_case("  t" => "  ".to_owned(); "Starting to type something")]
    #[test_case("  toto " => "  toto ".to_owned(); "Starting spaces and complete word")]
    #[test_case("  toto" => "  ".to_owned(); "Only starting spaces")]
    #[test_case("  toto tata" => "  toto ".to_owned(); "White spaces, word and white space")]
    #[test_case("  toto  tata" => "  toto  ".to_owned(); "White spaces, word and white spaces")]
    fn test_extract_completed_command(s: &str) -> String {
        super::inner::extract_completed_command_part(s)
    }

    #[test_case("" => false; "empty string")]
    #[test_case("  " => false; "only spaces")]
    #[test_case("toto" => false; "Missing space at the end")]
    #[test_case(" toto" => false; "Space before and missing space after word")]
    #[test_case("  toto" => false; "Spaces before and missing space after word")]
    #[test_case("  toto " => true; "Basic case ok")]
    #[test_case("  toto  " => true; "Basic case with multiple spaces at the end")]
    fn test_is_command_typed(s: &str) -> bool {
        super::inner::is_command_typed(s)
    }
}
