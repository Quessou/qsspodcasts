#[derive(Default)]
pub(crate) struct ReadonlyModalContext {
    pub(crate) content: Option<Vec<&'static str>>,
}
