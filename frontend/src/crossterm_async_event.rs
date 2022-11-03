use crossterm::event::poll as crossterm_poll;
use std::time::Duration;

use tokio::time::{sleep, timeout};

pub async fn poll(d: Duration) -> Result<bool, ()> {
    let mdr = async {
        loop {
            let a = crossterm_poll(Duration::from_secs(0));
            if a.is_ok() && a.unwrap() {
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
        true
    };

    match timeout(d, mdr).await {
        Ok(b) => Ok(b),
        Err(_) => Err(()),
    }
}
