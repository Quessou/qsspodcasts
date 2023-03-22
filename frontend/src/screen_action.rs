#[derive(PartialEq)]
pub enum ScreenAction {
    TypingCommand,
    ScrollingOutput,
    ScrollingLogs,
    ScrollingModalWindow,
}
