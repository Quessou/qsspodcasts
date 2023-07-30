#[derive(Clone, Copy, PartialEq)]
pub enum ScreenAction {
    TypingCommand,
    ScrollingOutput,
    ScrollingLogs,
    ScrollingModalWindow,
    ShowingReadOnlyModalWindow,
}
