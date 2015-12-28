use toolkit::canvas::Canvas;
use core::bindings::Key;

pub trait Widget<'a> {
    type Context;
    type View: View + 'a;

    fn enabled(&self, context: Self::Context) -> bool {
        true
    }

    fn view(&'a self, context: Self::Context) -> Self::View;

    fn on_keypress(&'a mut self, context: Self::Context, canvas: Canvas, key: Key) -> bool {
        false
    }

    fn focus(&mut self, context: Self::Context) {
    }

    fn unfocus(&mut self, context: Self::Context) {
    }
}

pub trait View {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(&self, canvas: Canvas);
}
