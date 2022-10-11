use super::command_output::CommandOutput;

pub fn build_from_string(s: &str) -> CommandOutput {
    CommandOutput::new(vec![Box::new(s.to_string())])
}
