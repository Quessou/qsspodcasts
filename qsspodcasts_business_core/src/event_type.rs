// TODO: Add a more concrete hash type ?
type Hash = String;

#[derive(Debug)]
pub enum EventType {
    PodcastFinished(Hash),
    PodcastLaunched(String),
}
