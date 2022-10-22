use crossterm::event::poll as crossterm_poll;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::time::timeout;

// TODO : find more relevant names when I'll understand what I'm doing
struct Polled {}

impl Polled {
    pub fn new() -> Polled {
        Polled {}
    }
}

impl Future for Polled {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let poll_status = crossterm_poll(Duration::from_secs(0));

        if poll_status.is_ok() && poll_status.unwrap() {
            return Poll::Ready(true);
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

pub async fn poll(d: Duration) -> Result<bool, ()> {
    let polled = Polled::new();
    match timeout(d, polled).await {
        Ok(b) => Ok(b),
        Err(_) => Err(()),
    }
}
