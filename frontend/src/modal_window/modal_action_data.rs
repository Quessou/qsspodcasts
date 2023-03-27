use super::modal_actionable::ModalActionable;
use data_transport::DataSender;

pub(crate) struct ModalActionData<'a, T>
where
    T: ModalActionable,
{
    pub action: String,
    pub actionable: &'a T,
    pub callback: Box<dyn Fn(&T, String, DataSender<String>)>,
}
