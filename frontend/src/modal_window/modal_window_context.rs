use std::cell::RefCell;

use tui::widgets::ListState;

use super::modal_action::ModalAction;

#[derive(Default)]
pub struct ModalWindowContext {
    pub modal_actions: Option<Vec<ModalAction>>,
    pub modal_actions_list_state: Option<RefCell<ListState>>,
}

impl ModalWindowContext {
    pub fn reset(&mut self, modal_actions: Option<Vec<ModalAction>>) {
        match modal_actions {
            Some(v) => {
                self.modal_actions = Some(v);
                self.modal_actions_list_state = Some(RefCell::new(ListState::default()));
                if self.modal_actions.as_ref().unwrap().is_empty() {
                    self.modal_actions_list_state
                        .as_mut()
                        .unwrap()
                        .get_mut()
                        .select(Some(0));
                }
            }
            None => {
                self.modal_actions = None;
                self.modal_actions_list_state = None;
            }
        }
    }
}
