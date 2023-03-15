use crate::Autocompleter;
use crate::AutocompletionResponse;

use data_transport::{DataReceiver, DataSender};

pub struct AutocompleterMessageProxy {
    autocompleter: Autocompleter,
    request_receiver: DataReceiver<String>,
    response_sender: DataSender<AutocompletionResponse>,
}

impl AutocompleterMessageProxy {
    pub fn new(
        autocompleter: Autocompleter,
        request_receiver: DataReceiver<String>,
        response_sender: DataSender<AutocompletionResponse>,
    ) -> Self {
        Self {
            autocompleter,
            request_receiver,
            response_sender,
        }
    }

    pub async fn run(&mut self) {
        // TODO : Handle close
        while let Some(ref request) = self.request_receiver.receive().await {
            let response = self.autocompleter.autocomplete(request);
            self.response_sender
                .send(response)
                .await
                .expect("Response sending failed");
        }
        self.request_receiver.close();
    }
}
