use std::cell::RefCell;

use tui::widgets::ListState;

use super::modal_action::ModalAction;

#[derive(Default)]
pub struct InteractableModalWindowContext {
    pub modal_actions: Option<Vec<ModalAction>>,
    pub modal_actions_list_state: Option<RefCell<ListState>>,
}

impl InteractableModalWindowContext {
    pub fn reset(&mut self, modal_actions: Option<Vec<ModalAction>>) {
        match modal_actions {
            Some(v) => {
                self.modal_actions = Some(v);
                let default_index = if self.modal_actions.as_ref().unwrap().is_empty() {
                    None
                } else {
                    Some(0)
                };
                self.modal_actions_list_state = Some(RefCell::new(
                    ListState::default().with_selected(default_index),
                ));
            }
            None => {
                self.modal_actions = None;
                self.modal_actions_list_state = None;
            }
        }
    }
}
