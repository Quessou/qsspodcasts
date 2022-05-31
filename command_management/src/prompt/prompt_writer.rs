use super::minimalistic_prompt_generator::MinimalisticPromptGenerator;
use super::prompt_generator::PromptGenerator;
use tokio::io as tokioIo;

use tokio::io::{AsyncWriteExt, BufWriter, Stdout};

pub struct PromptWriter<PG: PromptGenerator> {
    stdout_writer: BufWriter<Stdout>,
    prompt_generator: Box<PG>,
}

impl<PG: PromptGenerator> PromptWriter<PG> {
    pub fn new(prompt_generator: Box<PG>) -> PromptWriter<PG> {
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

impl Default for PromptWriter<MinimalisticPromptGenerator> {
    fn default() -> PromptWriter<MinimalisticPromptGenerator> {
        PromptWriter::new(Box::new(MinimalisticPromptGenerator::default()))
    }
}
