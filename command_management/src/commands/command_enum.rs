use chrono;
use url::Url;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Play,
    Pause,
    Exit,
    Help(Option<String>),
    ListPodcasts,
    ListEpisodes,
    Search,
    See,
    VolumeUp,
    VolumeDown,
    AddRss(Url),
    DeleteRss,
    Select(String),
    Advance(chrono::Duration),
    GoBack(chrono::Duration),
}
