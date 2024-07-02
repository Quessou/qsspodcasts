use std::str::FromStr;

use chrono;
use url as extern_url;

use strum_macros::{Display, EnumIter};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CommandUrl(pub extern_url::Url);

impl Default for CommandUrl {
    fn default() -> Self {
        CommandUrl(extern_url::Url::from_str("https://www.example.com").unwrap())
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct CommandDuration(pub chrono::Duration);

impl Default for CommandDuration {
    fn default() -> Self {
        Self(chrono::Duration::nanoseconds(0))
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Play(Option<String>),
    Pause,
    Exit,
    Help(Option<String>),
    ListPodcasts,
    ListEpisodes(Option<String>),
    Search,
    See,
    VolumeUp,
    VolumeDown,
    AddRss(CommandUrl),
    DeleteRss(String),
    Select(String),
    Advance(CommandDuration),
    GoBack(CommandDuration),
    MarkAsFinished,
    LatestPodcasts,
}
