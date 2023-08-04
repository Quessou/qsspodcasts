#[derive(Clone)]
pub struct CommandHelp {
    pub command_name: &'static str,
    pub sample: &'static str,
    pub description: &'static str,
}
