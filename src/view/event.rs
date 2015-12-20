use toolkit::Canvas;
use core::keymap::Key;

pub trait OnKeypress {
    type Context;
    fn on_keypress(&mut self, context: &Self::Context, canvas: Canvas, key: Key) -> bool;
}
