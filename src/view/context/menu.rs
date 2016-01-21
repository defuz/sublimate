use core::Core;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::bindings::Key;
use core::menu::{Menu, MenuItem};

use toolkit::*;

use unicode_width::UnicodeWidthStr;

use view::modal::ModalView;

use view::theme::*;

use super::item::{ContextMenuItem, ContextMenuItemView};

#[derive(Debug)]
pub struct ContextMenu {
    focused: Option<usize>,
    items: Vec<ContextMenuItem>
}

#[derive(Debug)]
pub struct ContextMenuView<'a> {
    pub views: Vec<ContextMenuItemView<'a>>
}

impl ContextMenu {
    pub fn new(items: Menu) -> Self {
        ContextMenu {focused: None, items: items.into_iter().map(From::from).collect()}
    }

    fn focused(&mut self) -> Option<&mut ContextMenuItem> {
        match self.focused {
            Some(index) => Some(&mut self.items[index]),
            None => None
        }
    }

    fn focus_prev(&mut self, core: &Core) {
        let skip = self.focused.map(|i| self.items.len() - i).unwrap_or(0);
        self.unfocus(core);
        self.focused = self.items
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(skip)
            .filter(|&(_, ref item)| item.enabled((core, false)))
            .next()
            .map(|(i, _)| i);
    }

    fn focus_next(&mut self, core: &Core) {
        let skip = self.focused.map(|i| i + 1).unwrap_or(0);
        self.unfocus(core);
        self.focused = self.items
            .iter()
            .enumerate()
            .cycle()
            .skip(skip)
            .filter(|&(_, ref item)| item.enabled((core, false)))
            .next()
            .map(|(i, _)| i);
    }
}

impl<'a> Widget<'a> for ContextMenu {
    type Context = &'a Core;
    type View = ContextMenuView<'a>;

    fn view(&'a self, core: &'a Core) -> ContextMenuView<'a> {
        ContextMenuView { views: self.items
            .iter()
            .enumerate()
            .map(|(i, item)| item.view((core, Some(i) == self.focused)))
            .collect()
        }
    }

    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        let mut processed = false;
        if let Some(ref mut item) = self.focused() {
            if item.on_keypress((core, true), canvas, key) {
                processed = true
            }
        }
        if processed {
            self.view(core).render(canvas);
            return true
        }
        if key == Key::Up {
            self.focus_prev(core);
            self.view(core).render(canvas);
            return true
        }
        if key == Key::Down {
            self.focus_next(core);
            self.view(core).render(canvas);
            return true
        }
        return false
    }

    fn focus(&mut self, core: &Core) {
        if self.focused == None {
            self.focus_next(core);
        }
    }

    fn unfocus(&mut self, core: &Core) {
        if let Some(item) = self.focused() {
            item.unfocus((core, true));
        }
        self.focused = None;
    }
}

impl<'a> View for ContextMenuView<'a> {
    fn width(&self) -> usize {
        self.views.iter().map(|view| view.width()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.views.len()
    }

    fn render(&self, mut canvas: Canvas) {
        for view in self.views.iter() {
            let h = view.height();
            if h > canvas.height() {
                break;
            }
            view.render(canvas.cut_top(h));
        }
    }
}
