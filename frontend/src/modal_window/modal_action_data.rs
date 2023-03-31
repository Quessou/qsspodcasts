use super::modal_action_callbacks::ModalActionCallback;
use super::modal_actionable::ModalActionable;

pub(crate) struct ModalActionData<'a, T>
where
    T: ModalActionable + 'a,
{
    pub action: String,
    pub actionable: &'a T,
    pub callback: Box<ModalActionCallback<'a, T>>,
}

impl<'a, T> ModalActionData<'a, T>
where
    T: ModalActionable + 'a,
{
    pub fn new(
        action: String,
        actionable: &'a T,
        callback: Box<ModalActionCallback<'a, T>>,
    ) -> ModalActionData<'a, T> {
        ModalActionData {
            action,
            actionable,
            callback,
        }
    }
}
