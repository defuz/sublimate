use std::cell::Cell;
use std::fmt::Debug;
use vec_map::VecMap;
use std::marker::PhantomData;

use ncurses::{newwin, new_panel, PANEL, del_panel, update_panels, doupdate};


use core::Core;
use core::keymap::Key;

use toolkit::*;

use view::window::{Window, Context};
use view::context::ContextMenu;
use view::event::OnKeypress;

#[derive(Debug)]
pub enum ModalPosition {
    AboveLeft,
    AboveRight,
    UnderLeft,
    UnderRight
}

#[derive(Debug)]
pub struct ModalManager {
    modals: Vec<ContextMenu>,
    panels: VecMap<PANEL>
}

#[derive(Debug)]
pub struct Modal<C, V: View<C>> {
    position: ModalPosition,
    _phantom: PhantomData<C>,
    panel: Cell<Option<PANEL>>,
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
        self.panel.set(Some(panel));
        self.content.render(context, canvas);
        update_panels();
        doupdate();
    }

    pub fn hide(&self) {
        if let Some(panel) = self.panel.get() {
            del_panel(panel);
            self.panel.set(None)
        }
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

impl ModalManager {
    pub fn new(modals: Vec<ContextMenu>) -> ModalManager {
        ModalManager { modals: modals, panels: VecMap::new() }
    }

    pub fn show_modal_window(&mut self, id: usize, core: &Core, position: ModalPosition) {
        // let ref view = self.modals[id];
        // let (canvas, panel) = position.get_window(view.width(core), view.height(core));
        // view.render(core, canvas);
        // self.panels.insert(id, panel);
        // self.update();
    }

    pub fn hide_modal_window(&mut self, id: usize) {
        if let Some(panel) = self.panels.remove(&id) {
            del_panel(panel);
        }
    }

    pub fn clear(&mut self) {
        for panel in self.panels.values() {
            del_panel(*panel);
        }
        self.panels.clear();
    }

    pub fn replace_modal_window(&mut self, id: usize, core: &Core, position: ModalPosition) {
        self.clear();
        self.show_modal_window(id, core, position);
    }

    pub fn update(&self) {
        update_panels();
        doupdate();
    }
}

impl<'c> OnKeypress<&'c mut Core> for ModalManager {
    fn on_keypress(&mut self, core: &'c mut Core, canvas: Canvas, key: Key) -> bool {
        // let context = Context { core: core, modals: self};
        // for menu in self.modals.iter().rev() {
        //     if menu.on_keypress(context, canvas, key) {
        //         return true;
        //     }
        // }
        false
        // self.base.on_keypress(core, canvas, key)
    }
}
