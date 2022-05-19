use super::prompt_generator::PromptGenerator;

#[derive(Default)]
pub struct MinimalisticPromptGenerator {}

impl MinimalisticPromptGenerator {
    pub fn new() -> MinimalisticPromptGenerator {
        MinimalisticPromptGenerator::default()
    }
}

impl PromptGenerator for MinimalisticPromptGenerator {
    fn generate_prompt(&self) -> String {
        String::from(">>> ")
    }
}
