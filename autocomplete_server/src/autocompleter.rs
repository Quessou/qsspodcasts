use crate::AutocompletionResponse;
use command_management::autocompletion::autocompletion_command_data::AutocompletionCommandData;

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

    pub fn autocomplete_command(&self, command: String) -> AutocompletionResponse {
        AutocompletionResponse::default()
    }

    pub fn autocomplete_hash(&self, hash: String) -> AutocompletionResponse {
        AutocompletionResponse::default()
    }
}
