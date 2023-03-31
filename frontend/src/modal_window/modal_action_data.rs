use super::modal_action_callbacks::BuildCommandCallback;
use super::modal_actionable::ModalActionable;

pub(crate) struct ModalActionData<'a, T>
where
    T: ModalActionable + 'a,
{
    pub action: String,
    pub actionable: &'a T,
    pub build_command_callback: Box<BuildCommandCallback<'a, T>>,
}

impl<'a, T> ModalActionData<'a, T>
where
    T: ModalActionable + 'a,
{
    pub fn new(
        action: String,
        actionable: &'a T,
        build_command_callback: Box<BuildCommandCallback<'a, T>>,
    ) -> ModalActionData<'a, T> {
        ModalActionData {
            action,
            actionable,
            build_command_callback,
        }
    }
}
