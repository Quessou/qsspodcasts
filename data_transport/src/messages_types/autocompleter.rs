pub enum MessageType {
    HashUpdate(Vec<String>),
    AutocompletionRequest(String),
    Exit,
}
