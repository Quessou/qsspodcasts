use super::command_help_library::{CommandHelpLibrary, CommandHelpMap};
use super::command_help_register::CommandHelpRegister;

pub fn get_command_help_library() -> CommandHelpLibrary {
    let map =
    CommandHelpMap::from(
        [
            ("help",
                CommandHelpRegister::new("help", "help [COMMAND_NAME]",
            "Displays help",
             Some("If a command name is specified, displays a detailed help about the given command. Else returns a short help about all commands")
            )
        ),
        ("exit",
            CommandHelpRegister::new("exit", "exit",
        "Exits",
         None
        )
    )
        ]
    );
    CommandHelpLibrary::new(map)
}
