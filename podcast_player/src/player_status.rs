use crate::duration_wrapper::DurationWrapper;

pub enum PlayerStatus {
    Stopped,
    Paused(DurationWrapper, DurationWrapper, u8),
    Playing(DurationWrapper, DurationWrapper, u8),
}
