use super::minimalistic_prompt_generator::MinimalisticPromptGenerator;
use super::prompt_generator::PromptGenerator;
use tokio::io as tokioIo;

use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

pub struct PromptWriter {
    stdout_writer: BufWriter<Stdout>,
    prompt_generator: Box<dyn PromptGenerator>,
}

impl PromptWriter {
    pub fn new(prompt_generator: Box<dyn PromptGenerator>) -> PromptWriter {
        PromptWriter {
            stdout_writer: tokioIo::BufWriter::new(tokioIo::stdout()),
            prompt_generator,
        }
    }

    pub async fn write_prompt(&mut self) {
        let prompt = self.prompt_generator.generate_prompt();
        self.stdout_writer
            .write_all(prompt.as_bytes())
            .await
            .expect("Writing prompt failed");
        self.stdout_writer.flush().await.expect("Flushing failed");
    }
}

impl Default for PromptWriter {
    fn default() -> PromptWriter {
        PromptWriter::new(Box::new(MinimalisticPromptGenerator::default()))
    }
}
