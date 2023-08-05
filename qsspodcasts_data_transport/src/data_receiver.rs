use log::error;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;

/// Dumb wrapper on a Receiver end of a channel
/// Mostly here only to ensure that if I do not want to use channels anymore, the transition will be as painless as possible.
pub struct DataReceiver<T> {
    receiver: Receiver<T>,
}

impl<T> DataReceiver<T> {
    pub fn new(receiver: Receiver<T>) -> DataReceiver<T> {
        DataReceiver { receiver }
    }

    pub fn close(&mut self) {
        self.receiver.close()
    }

    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }

    pub fn try_receive(&mut self) -> Result<T, TryRecvError> {
        match self.receiver.try_recv() {
            Ok(d) => Ok(d),
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    }
}
