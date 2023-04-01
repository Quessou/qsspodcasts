use data_transport::DataSender;

use super::modal_action_callbacks::CallbackReturnType;
use super::modal_action_data::ModalActionData;
use super::modal_actionable::ModalActionable;

pub struct ModalAction {
    pub action: String,
    pub command: String,
    pub command_sender: DataSender<String>,
}

impl ModalAction {
    pub async fn call(&mut self) -> CallbackReturnType {
        self.send().await
    }

    pub async fn send(&mut self) -> CallbackReturnType {
        let f = self.command_sender.send(self.command.clone());
        f.await
    }
}

impl<'a, T> From<(ModalActionData<'a, T>, DataSender<String>)> for ModalAction
where
    T: ModalActionable,
{
    fn from(value: (ModalActionData<'a, T>, DataSender<String>)) -> Self {
        ModalAction {
            action: value.0.action,
            command: (value.0.build_command_callback)(value.0.actionable),
            command_sender: value.1,
        }
    }
}
