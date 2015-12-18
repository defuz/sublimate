use toolkit::Canvas;
use core::keymap::Key;

pub trait OnKeypress<C> {
    fn on_keypress(&mut self, context: &C, canvas: Canvas, key: Key) -> bool;
}
