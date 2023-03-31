use data_transport::DataSender;

use super::modal_action_callbacks::ModalActionCallback;
use super::modal_action_data::ModalActionData;
use super::modal_actionable::ModalActionable;

use super::modal_action_callbacks::CallbackReturnType;

pub(crate) struct ModalAction<'a, T>
where
    T: ModalActionable + 'a,
{
    pub action: String,
    pub actionable: &'a T,
    pub command_sender: DataSender<String>,
    pub callback: Box<ModalActionCallback<'a, T>>,
}

impl<'a, T> ModalAction<'a, T>
where
    T: ModalActionable + 'a,
{
    pub async fn call<'c: 'a>(&'c mut self) -> CallbackReturnType {
        (self.callback)(self.actionable, &mut self.command_sender).await // Check this : https://practice.rs/lifetime/advance.html
    }
}

impl<'a, T> From<(ModalActionData<'a, T>, DataSender<String>)> for ModalAction<'a, T>
where
    T: ModalActionable,
{
    fn from(value: (ModalActionData<'a, T>, DataSender<String>)) -> Self {
        ModalAction {
            action: value.0.action,
            actionable: value.0.actionable,
            command_sender: value.1,
            callback: value.0.callback,
        }
    }
}
