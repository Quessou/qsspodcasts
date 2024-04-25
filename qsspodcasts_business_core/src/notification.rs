use std::fmt::Display;

use super::event_type::EventType;

#[derive(Debug)]
pub enum Notification {
    Message(String),
    Event(EventType),
}
