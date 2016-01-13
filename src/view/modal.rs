use std::cell::Cell;

use ncurses::{newwin, new_panel, PANEL, del_panel, update_panels, doupdate};

use core::bindings::Key;

use toolkit::*;

#[derive(Debug, Clone, Copy)]
pub enum ModalPosition {
    AboveLeft,
    AboveRight,
    UnderLeft,
    UnderRight,
    RightTop,
}

#[derive(Debug)]
pub struct Modal<T> {
    position: ModalPosition,
    panel: Cell<Option<(PANEL, Canvas)>>,
    content: T
}

#[derive(Debug)]
pub struct ModalView<'a, T: View> {
    position: ModalPosition,
    panel: &'a Cell<Option<(PANEL, Canvas)>>,
    view: T
}

impl<T> Modal<T> {
    pub fn new(content: T, position: ModalPosition) -> Modal<T> {
        Modal {
            position: position,
            panel: Cell::new(None),
            content: content
        }
    }

    pub fn hide(&self) {
        if let Some((panel, _)) = self.panel.get() {
            del_panel(panel);
            self.panel.set(None)
        }
    }
}

impl<'a, T: Widget<'a>> Widget<'a> for Modal<T> {
    type Context = T::Context;
    type View = ModalView<'a, T::View>;

    fn view(&'a self, context: Self::Context) -> Self::View {
        ModalView {
            position: self.position,
            panel: &self.panel,
            view: self.content.view(context)
        }
    }

    fn on_keypress(&'a mut self, core: Self::Context, _: Canvas, key: Key) -> bool {
        let r = self.content.on_keypress(core, self.panel.get().unwrap().1, key);
        if r {
            update_panels();
            doupdate();
        }
        r
    }

    fn focus(&mut self, context: Self::Context) {
        self.content.focus(context);
    }

    fn unfocus(&mut self, context: Self::Context) {
        self.content.unfocus(context);
        self.hide();
    }
}

impl<'a, T: View> View for ModalView<'a, T> {
    fn width(&self) -> usize {
        self.view.width()
    }

    fn height(&self) -> usize {
        self.view.height()
    }

    fn render(&self, base: Canvas) {
        if let Some((panel, _)) = self.panel.get() {
            del_panel(panel);
        }
        let (canvas, panel) = self.position.get_window(base, self.width(), self.height());
        self.panel.set(Some((panel, canvas)));
        self.view.render(canvas);
        update_panels();
        doupdate();
    }
}

impl ModalPosition {
    fn get_window(&self, base: Canvas, w: usize, h: usize) -> (Canvas, PANEL) {
        let (x, y) = match *self {
            ModalPosition::UnderLeft => (base.x0 + base.x1, base.y0 + base.y2),
            ModalPosition::RightTop  => (base.x0 + base.x2, base.y0 + base.y1),
            _ => unimplemented!()
        };
        let win = newwin(h as i32, w as i32, y as i32, x as i32);
        (Canvas {win: win, x0: x, y0: y, x1: 0, y1: 0, x2: w, y2: h}, new_panel(win))
    }
}
