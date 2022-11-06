use super::command_help::CommandHelp;

pub struct CommandHelpRegister {
    command_name: &'static str,
    sample: &'static str,
    short_description: &'static str,
    long_description: &'static str,
}

impl CommandHelpRegister {
    pub fn new(
        command_name: &'static str,
        sample: &'static str,
        short_description: &'static str,
        long_description: Option<&'static str>,
    ) -> CommandHelpRegister {
        let long_description = match long_description {
            Some(s) => s,
            None => short_description,
        };
        CommandHelpRegister {
            command_name,
            sample,
            short_description,
            long_description,
        }
    }

    pub fn get_short_help(&self) -> CommandHelp {
        CommandHelp {
            command_name: self.command_name,
            sample: self.sample,
            description: self.short_description,
        }
    }

    pub fn get_detailed_help(&self) -> CommandHelp {
        CommandHelp {
            command_name: self.command_name,
            sample: self.sample,
            description: self.long_description,
        }
    }
}
