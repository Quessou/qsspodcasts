use crate::AutocompletionResponse;
use command_management::autocompletion::autocompletion_command_data::AutocompletionCommandData;
use command_management::autocompletion::command_parameter_type::CommandParameterType;

mod inner {
    pub fn extract_completed_command_part(command: &str) -> String {
        let mut completed_command = command.split(' ').filter(|s| !s.is_empty());

        let _ = completed_command.next_back();
        completed_command
            .fold(String::default(), |mut i, s| {
                i.push(' ');
                i.push_str(s);
                i
            })
            .trim()
            .to_owned()
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

        self.commands
            .iter()
            .map(|c| c.command.to_string())
            .filter(|c| c.starts_with(to_be_completed))
            .collect()
    }

    pub fn autocomplete_hash(&self, to_be_completed: &str) -> Vec<String> {
        assert!(!to_be_completed.contains(' '));
        self.hashes
            .iter()
            .filter(|h| h.starts_with(to_be_completed))
            .cloned()
            .collect()
    }

    pub fn autocomplete(&self, line_to_be_autocompleted: &str) -> AutocompletionResponse {
        let to_be_autocompleted = line_to_be_autocompleted.trim();

        if to_be_autocompleted.is_empty() {
            return AutocompletionResponse::default();
        }

        let completed_command_part = inner::extract_completed_command_part(to_be_autocompleted);
        let to_be_autocompleted = to_be_autocompleted.split(' ').last().unwrap();

        let autocompletion_options = if !line_to_be_autocompleted.contains(' ') {
            self.autocomplete_command(to_be_autocompleted)
        } else {
            let typed_command = line_to_be_autocompleted.split(' ').next().unwrap();
            let command = self
                .commands
                .iter()
                .find(|c| c.command.to_string() == typed_command);

            if let Some(parameter_type) = command {
                let parameter_type = &parameter_type.parameter_type;
                if parameter_type.is_none() {
                    return AutocompletionResponse::default();
                }
                match command.unwrap().parameter_type.as_ref().unwrap() {
                    CommandParameterType::Hash => {
                        self.autocomplete_hash(to_be_autocompleted.split(' ').last().unwrap())
                    }
                    CommandParameterType::CommandName => {
                        self.autocomplete_command(to_be_autocompleted)
                    }
                    _ => unreachable!(),
                }
            } else {
                vec![]
            }
        };

        AutocompletionResponse {
            autocompletion_options: autocompletion_options
                .iter()
                .map(|o| {
                    let mut result = completed_command_part.clone();
                    result.push(' ');
                    result.push_str(o);
                    result.trim().to_owned()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;
    use command_management::commands::command_enum::Command;

    #[test_case("help li" => "help".to_owned(); "Rather regular case" )]
    #[test_case("help  li" => "help".to_owned(); "Edge case with several spaces in the string" )]
    #[test_case("help toto ha" => "help toto".to_owned(); "Case with several words" )]
    #[test_case("help  " => "".to_owned(); "Edge case with only spaces at the end of the string" )]
    fn test_extract_completed_command_part(to_be_completed: &str) -> String {
        inner::extract_completed_command_part(to_be_completed)
    }

    #[test]
    fn test_autocomplete_command() {
        let autocompleter = Autocompleter::new(vec![
            AutocompletionCommandData::new(Command::Exit, None),
            AutocompletionCommandData::new(Command::ListPodcasts, None),
            AutocompletionCommandData::new(Command::ListEpisodes, None),
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
}
