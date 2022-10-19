use url::Url;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Command {
    Play,
    Pause,
    Exit,
    Help,
    ListPodcasts,
    ListEpisodes,
    Search,
    See,
    VolumeUp,
    VolumeDown,
    AddRss(Url),
    DeleteRss,
    Select(String),
}
