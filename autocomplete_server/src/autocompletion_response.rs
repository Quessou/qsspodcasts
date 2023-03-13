pub struct AutocompletionResponse {
    pub autocompletion_options: Vec<String>,
}

impl AutocompletionResponse {
    pub fn new(autocompletion_options: Vec<String>) -> Self {
        Self {
            autocompletion_options,
        }
    }
}

impl Default for AutocompletionResponse {
    fn default() -> Self {
        Self::new(vec![])
    }
}
