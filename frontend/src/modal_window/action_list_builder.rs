use super::modal_action::ModalAction;
use super::modal_actionable::ModalActionable;
use data_transport::DataSender;
use std::marker::PhantomData;

pub(crate) struct ActionListBuilder<T>
where
    T: ModalActionable,
{
    command_sender: DataSender<String>,
    _marker: PhantomData<T>,
}

impl<T> ActionListBuilder<T>
where
    T: ModalActionable,
{
    pub fn new(command_sender: DataSender<String>) -> ActionListBuilder<T> {
        ActionListBuilder {
            command_sender,
            _marker: PhantomData,
        }
    }

    pub fn build_action_list(&self, actionable: &T) -> Vec<ModalAction<T>> {
        let action_list = actionable.get_action_list();
    }
}
