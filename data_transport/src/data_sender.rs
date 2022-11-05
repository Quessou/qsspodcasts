use log::error;
use tokio::sync::mpsc::Sender;

/// Dumb wrapper on a Sender end of a channel
/// Mostly here only to ensure that if I do not want to use channels anymore, the transition will be as painless as possible.
pub struct DataSender<T> {
    sender: Sender<T>,
}

impl<T> DataSender<T> {
    pub fn new(sender: Sender<T>) -> DataSender<T> {
        DataSender { sender }
    }

    pub async fn send(&mut self, data: T) -> Result<(), ()> {
        if self.sender.is_closed() {
            error!("Trying to send data on closed channel");
            return Err(());
        }

        match self.sender.send(data).await {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn is_closed(&mut self) -> bool {
        self.sender.is_closed()
    }
}
