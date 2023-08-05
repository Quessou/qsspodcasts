use crate::screen_context::ScreenContext;
use tui::backend::Backend;
use tui::Frame;

pub trait UiDrawer {
    fn draw_ui<B: Backend>(&mut self, f: &mut Frame<B>, context: &ScreenContext);
}
