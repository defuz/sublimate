
use std::cmp;
use std::borrow::Borrow;
use std::convert::AsRef;
use std::iter::Iterator;
use std::fmt::Debug;

struct Menubar;

struct Window {
    menubar: Menubar
}


trait OnKeypress<C> {
    fn on_keypress(&mut self, context: C);
}

impl<'c> OnKeypress<&'c mut Window> for Menubar {
    fn on_keypress(&mut self, context: &'c mut Window) {

    }
}

impl Window {
    fn on_keypress(&mut self) {
        self.menubar.on_keypress(self)
    }
}


fn main() {

}
