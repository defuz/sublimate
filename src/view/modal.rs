use std::fmt::Debug;
use vec_map::VecMap;

use ncurses::{newwin, new_panel, PANEL, del_panel, update_panels, doupdate};


use core::Core;
use core::keymap::Key;

use toolkit::*;

use view::context::ContextMenu;
use view::event::OnKeypress;

#[derive(Debug)]
pub enum ModalPosition {
    AboveLeft(Canvas),
    AboveRight(Canvas),
    UnderLeft(Canvas),
    UnderRight(Canvas)
}

#[derive(Debug)]
pub struct ModalManager {
    modals: Vec<ContextMenu>,
    panels: VecMap<PANEL>
}

struct Modal;

// impl Modal {
//     fn show() {

//     }

//     fn hide() {

//     }
// }

impl ModalPosition {
    fn get_window(&self, w: usize, h: usize) -> (Canvas, PANEL) {
        match *self {
            ModalPosition::UnderLeft(canvas) => {
                // FIXME: use absolute coordinates here
                let win = newwin(h as i32, w as i32, canvas.y2 as i32, canvas.x1 as i32);
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
        let ref view = self.modals[id];
        let (canvas, panel) = position.get_window(view.width(core), view.height(core));
        view.render(core, canvas);
        self.panels.insert(id, panel);
        self.update();
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

// impl<C> OnKeypress<C> for ModalManager<C> {
//     fn on_keypress(&mut self, core: C, canvas: Canvas, key: Key) -> bool {
//         false
//         // self.base.on_keypress(core, canvas, key)
//     }
// }
