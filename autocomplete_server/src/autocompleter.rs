use crate::AutocompletionResponse;
use command_management::autocompletion::autocompletion_command_data::AutocompletionCommandData;
use command_management::autocompletion::command_parameter_type::CommandParameterType;

mod inner {
    pub fn extract_completed_command_part(command: &str) -> String {
        let mut completed_command = command.split(' ').filter(|s| !s.is_empty());
        if completed_command.clone().count() > 1 {
            let _ = completed_command.next_back();
        }
        completed_command
            .fold(String::default(), |mut i, s| {
                i.push_str(" ");
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

    pub fn autocomplete_command(&self, to_be_completed: &str) -> AutocompletionResponse {
        let completed_command_part = inner::extract_completed_command_part(to_be_completed);

        let possible_commands = self
            .commands
            .iter()
            .map(|c| c.command.to_string())
            .filter(|c| c.starts_with(&command));
        AutocompletionResponse::new(possible_commands.collect())
    }

    pub fn autocomplete_hash(&self, to_be_completed: &str) -> AutocompletionResponse {
        let completed_command_part = inner::extract_completed_command_part(to_be_completed);
        let hash = to_be_completed.split(' ').last().unwrap();
        let matching_hashes = self
            .hashes
            .iter()
            .filter(|h| h.starts_with(&hash))
            .map(|h| h.clone());
        AutocompletionResponse::new(
            matching_hashes
                .map(|h| {
                    completed_command_part.clone().push_str(&h);
                    h
                })
                .collect(),
        )
    }

    pub fn autocomplete(&self, to_be_autocompleted: &String) -> AutocompletionResponse {
        let to_be_autocompleted = to_be_autocompleted.trim();
        if !to_be_autocompleted.contains(" ") {
            self.autocomplete_command(to_be_autocompleted)
        } else {
            let typed_command = to_be_autocompleted.split(' ').next().unwrap();
            let command = self
                .commands
                .iter()
                .find(|c| c.command.to_string() == typed_command);
            let response = if command.is_none() {
                AutocompletionResponse::default()
            } else {
                match command.unwrap().parameter_type.as_ref().unwrap() {
                    CommandParameterType::Hash => self.autocomplete_hash(to_be_autocompleted),
                    CommandParameterType::CommandName => {
                        self.autocomplete_command(to_be_autocompleted)
                    }
                    _ => unreachable!(),
                }
            };
            response
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
    #[test_case("help  " => "help".to_owned(); "Edge case with only spaces at the end of the string" )]
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
        let command_to_be_completed = String::from("ex");

        let autocomplete_choices = autocompleter.autocomplete_command(&command_to_be_completed);
        assert_eq!(autocomplete_choices.autocompletion_options.len(), 1);
        assert_eq!(autocomplete_choices.autocompletion_options[0], "exit");

        let command_to_be_completed = String::from("lis");

        let autocomplete_choices = autocompleter.autocomplete_command(&command_to_be_completed);
        assert_eq!(autocomplete_choices.autocompletion_options.len(), 2);
    }
}
