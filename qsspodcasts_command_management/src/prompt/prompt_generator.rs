#[allow(dead_code)]
pub trait PromptGenerator {
    fn generate_prompt(&self) -> String;
}
