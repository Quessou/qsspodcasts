pub use podcast_management::style::stylized::{Style, Stylized, StylizedContent};

pub struct CommandOutput {
    pub output: Vec<Box<dyn Stylized>>,
}

impl CommandOutput {
    pub fn new(output: Vec<Box<dyn Stylized>>) -> CommandOutput {
        CommandOutput { output }
    }
}

impl Stylized for CommandOutput {
    fn to_stylized(&self) -> StylizedContent {
        self.output
            .iter()
            .map(|s| s.to_stylized())
            .flatten()
            .collect::<Vec<(&str, Option<Vec<Style>>)>>()
    }
}
