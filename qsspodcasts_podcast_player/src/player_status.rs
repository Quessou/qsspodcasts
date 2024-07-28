use crate::duration_wrapper::DurationWrapper;

pub enum PlayerStatus {
    Stopped(Option<(DurationWrapper, DurationWrapper, u8)>),
    Paused(DurationWrapper, DurationWrapper, u8),
    Playing(DurationWrapper, DurationWrapper, u8),
}
