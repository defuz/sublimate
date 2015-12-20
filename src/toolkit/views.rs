use toolkit::canvas::Canvas;
use core::keymap::Key;

pub trait Widget<'a> {
    type Context;
    type View: View + 'a;

    fn view(&'a self, context: &Self::Context) -> Self::View;
    fn on_keypress(&mut self, context: &Self::Context, canvas: Canvas, key: Key) -> bool;
}

pub trait View {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(&self, canvas: Canvas);
}
