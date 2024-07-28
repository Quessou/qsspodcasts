use super::command_help_library::{CommandHelpLibrary, CommandHelpMap};
use super::command_help_register::CommandHelpRegister;
use crate::commands::command_enum::{Command, CommandDuration, CommandUrl};

pub fn get_command_help_library() -> CommandHelpLibrary {
    let map =
    CommandHelpMap::from(
        [
            (Command::Help(None).to_string(),
                CommandHelpRegister::new("help", "help [COMMAND_NAME]",
            "Displays help",
             Some("If a command name is specified, displays a detailed help about the given command.
                  Else returns a short help about all commands")
            )),
        (Command::Exit.to_string(),
            CommandHelpRegister::new("exit", "exit",
        "Exits",
         None
        )),
    (Command::Play(None).to_string(),
     CommandHelpRegister::new("play", "play [HASH]",
     "Launches the podcast",
    Some("If no hash if given, resumes the selected podcast.
         If a hash is given, selects the associated podcast and launches it.")
    )),
    (Command::Pause.to_string(), CommandHelpRegister::new("pause", "pause", "Pauses the player",
    None)),
    (Command::ListPodcasts.to_string(),
     CommandHelpRegister::new("list_podcasts", "list_podcasts", "Lists all subscribed podcasts",
                                                                None)),
    (Command::ListEpisodes(None).to_string(),
     CommandHelpRegister::new("list_episodes", "list_episodes [HASH]", "Lists episodes and some information about them, including their hashes",
                                                                Some("If no hash is given, lists all episodes of all subscribed podcasts, sorted by release date.
                                                                If a podcast hash is given, lists all episodes for the given podcast.")),
    ),
    (Command::AddRss(CommandUrl::default()).to_string(), CommandHelpRegister::new("add_rss", "add_rss <URL>", "register the RSS feed whose URL is given in parameter", None)),
    (Command::DeleteRss(String::default()).to_string(), CommandHelpRegister::new("delete_rss", "delete_rss <HASH>", "Delete the RSS feed matching the podcast hash given in parameter", None)),
    (Command::Select(String::default()).to_string(), CommandHelpRegister::new("select", "select <HASH>", "Selects a podcast", Some("Selects a podcast, allowing to play it"))),
    (Command::Advance(CommandDuration::default()).to_string(), CommandHelpRegister::new("advance", "advance <DURATION>", "Advances the podcast of the given duration",
                                                                                        Some("Advances the podcast of the duration given in parameter.
                                                                                        The duration can be expressed a lot of ways, including :
                                                                                        - A number of seconds (e.g. : 40)
                                                                                        - Numbers separated by colons (e.g. : 2:30)
                                                                                        - A duration specified in an idomatic way (e.g. : 1h01m20s)"))),
    (Command::GoBack(CommandDuration::default()).to_string(), CommandHelpRegister::new("go_back", "go_back <DURATION>", "Goes back into the podcast of the given duration",
                                                                                        Some("Goes back in the podcast of the duration given in parameter.
                                                                                        The duration can be expressed a lot of ways, including :
                                                                                        - A number of seconds (e.g. : 40)
                                                                                        - Numbers separated by colons (e.g. : 2:30)
                                                                                        - A duration specified in an idomatic way (e.g. : 1h01m20s)"))),
    (Command::MarkAsFinished.to_string(),
     CommandHelpRegister::new("mark_as_finished", "mark_as_finished",
     "Marks current podcast as finished",
    Some("Marks the podcast currently selected as finished.")
    )),
    (Command::LatestPodcasts.to_string(),
     CommandHelpRegister::new("latest_podcasts", "latest_podcasts",
     "Displays podcasts that released today or yesterday",
     None
    )),
    (Command::VolumeUp(0).to_string(),
     CommandHelpRegister::new("volume_up", "volume_up <INTEGER>",
     "Sets the volume up of <INTEGER> percents",
     None
    )),
    (Command::VolumeDown(0).to_string(),
     CommandHelpRegister::new("volume_down", "volume_down <INTEGER>",
     "Sets the volume down of <INTEGER> percents",
     None
    )),
    (Command::SetVolume(0).to_string(),
     CommandHelpRegister::new("set_volume", "set_volume <INTEGER>",
     "Sets the volume to <INTEGER> percents of the max volume",
     None
    ))],

    );
    CommandHelpLibrary::new(map)
}
