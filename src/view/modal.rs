use std::cell::Cell;
use std::fmt::Debug;
use vec_map::VecMap;
use std::marker::PhantomData;

use ncurses::{newwin, new_panel, PANEL, del_panel, update_panels, doupdate};

use core::Core;
use core::keymap::Key;

use toolkit::*;

use view::event::OnKeypress;

#[derive(Debug)]
pub enum ModalPosition {
    AboveLeft,
    AboveRight,
    UnderLeft,
    UnderRight
}

#[derive(Debug)]
pub struct Modal<C, V: View<C>> {
    position: ModalPosition,
    _phantom: PhantomData<C>,
    panel: Cell<Option<(PANEL, Canvas)>>,
    content: V
}

impl<C, V> Modal<C, V> where C: Debug, V: View<C> {
    pub fn new(content: V, position: ModalPosition) -> Modal<C, V> {
        Modal {
            position: position,
            _phantom: PhantomData,
            panel: Cell::new(None),
            content: content
        }
    }

    pub fn render(&self, context: &C, base: Canvas) {
        self.hide();
        let (canvas, panel) = self.position.get_window(base, self.content.width(context), self.content.height(context));
        self.panel.set(Some((panel, canvas)));
        self.content.render(context, canvas);
        update_panels();
        doupdate();
    }

    pub fn hide(&self) {
        if let Some((panel, _)) = self.panel.get() {
            del_panel(panel);
            self.panel.set(None)
        }
    }

}

impl<C, V> OnKeypress<C> for Modal<C, V> where C: Debug, V: View<C>+OnKeypress<C> {
    fn on_keypress(&mut self, core: &C, base: Canvas, key: Key) -> bool {
        let r = self.content.on_keypress(core, self.panel.get().unwrap().1, key);
        if r {
            update_panels();
            doupdate();
        }
        r
    }
}

impl ModalPosition {
    fn get_window(&self, base: Canvas, w: usize, h: usize) -> (Canvas, PANEL) {
        match *self {
            ModalPosition::UnderLeft => {
                // FIXME: use absolute coordinates here
                let win = newwin(h as i32, w as i32, base.y2 as i32, base.x1 as i32);
                (Canvas {win: win, x1: 0, y1: 0, x2: w, y2: h}, new_panel(win))
            },
            _ => unimplemented!()
        }
    }
}
