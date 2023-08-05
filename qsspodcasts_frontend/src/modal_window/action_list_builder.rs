use super::modal_action::ModalAction;
use super::modal_actionable::ModalActionable;
use data_transport::DataSender;

pub struct ActionListBuilder {
    command_sender: DataSender<String>,
}

impl ActionListBuilder {
    pub fn new(command_sender: DataSender<String>) -> ActionListBuilder {
        ActionListBuilder { command_sender }
    }

    pub fn build_action_list<'a>(
        &'a self,
        actionable: &'a impl ModalActionable,
    ) -> Vec<ModalAction> {
        let action_list = actionable.get_action_list();
        action_list
            .into_iter()
            .map(|a| ModalAction::from((a, self.command_sender.clone())))
            .collect()
    }
}
