use strum_macros::Display;

#[derive(Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum Mp3PlayerState {
    Stopped,
    Buffering,
    Paused,
    Playing,
}

impl From<gstreamer_play::PlayState> for Mp3PlayerState {
    fn from(value: gstreamer_play::PlayState) -> Self {
        match value {
            gstreamer_play::PlayState::Stopped => Mp3PlayerState::Stopped,
            gstreamer_play::PlayState::Buffering => Mp3PlayerState::Buffering,
            gstreamer_play::PlayState::Paused => Mp3PlayerState::Paused,
            gstreamer_play::PlayState::Playing => Mp3PlayerState::Playing,
            _ => panic!(),
        }
    }
}
