use data_transport::DataSender;

use super::modal_action_data::ModalActionData;
use super::modal_actionable::ModalActionable;

pub(crate) struct ModalAction<'a, T>
where
    T: ModalActionable,
{
    pub action: String,
    pub actionable: &'a T,
    pub command_sender: DataSender<String>,
    pub callback: Box<dyn Fn(&T, String, DataSender<String>)>,
}

impl<T> From<(ModalActionData<'_, T>, DataSender<String>)> for ModalAction<'_, T>
where
    T: ModalActionable,
{
    fn from(value: (ModalActionData<'_, T>, DataSender<String>)) -> Self {
        todo!()
    }
}
