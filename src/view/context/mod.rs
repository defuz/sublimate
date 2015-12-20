use std::ops::IndexMut;

use core::Core;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::keymap::Key;
use core::menu::{Menu, MenuItem};

use toolkit::*;

pub mod view;

use self::view::{ContextMenuView, ContextMenuItemView, ButtonView, GroupView, ButtonState};

#[derive(Debug)]
pub struct ContextMenu {
    focused: Option<usize>,
    items: Vec<ContextMenuItem>
}

#[derive(Debug)]
enum ContextMenuItem {
    Divider,
    Button(Button),
    Group(Group),
}

#[derive(Debug)]
struct Button {
    caption: Option<String>,
    command: Command,
    is_checkbox: bool
}

impl Button {
    fn new(caption: Option<String>, command: Command, is_checkbox: bool) -> Button {
        Button { caption: caption, command: command, is_checkbox: is_checkbox }
    }

    fn view<'a>(&'a self, core: &Core, focused: bool) -> ButtonView<'a> {
        ButtonView {
            caption: match self.caption {
                Some(ref s) => s,
                None => "No caption"
            },
            hotkey: core.hotkeys.get_hotkeys(&self.command).map(|h| h.to_string()).unwrap_or_default(),
            state: if focused {
                ButtonState::Focused
            } else {
                ButtonState::Unfocused
            }
        }
    }
}

#[derive(Debug)]
struct Group {
    caption: String,
    modal: Modal<ContextMenu>,
    is_opened: bool
}

impl Group {
    fn new(caption: String, menu: Menu) -> Group {
        let modal = Modal::new(ContextMenu::new(menu), ModalPosition::RightTop);
        Group { caption: caption, modal: modal, is_opened: false }
    }

    fn view<'a>(&'a self, core: &Core, focused: bool) -> GroupView<'a> {
        GroupView {
            caption: &self.caption[..],
            is_focused: focused,
            modal: if self.is_opened {
                Some(self.modal.view(core))
            } else {
                None
            }
        }
    }
}

impl ContextMenuItem {
    fn enabled(&self, core: &Core) -> bool {
        match *self {
            ContextMenuItem::Button(..) | ContextMenuItem::Group(..) => true,
            ContextMenuItem::Divider => false
        }
    }
}

impl From<MenuItem> for ContextMenuItem {
    fn from(item: MenuItem) -> ContextMenuItem {
        match item {
            MenuItem::Button(caption, command, is_checkbox) =>
                ContextMenuItem::Button(Button::new(caption, command, is_checkbox)),
            MenuItem::Group(caption, menu) =>
                ContextMenuItem::Group(Group::new(caption, menu)),
            MenuItem::Divider => ContextMenuItem::Divider
        }
    }
}

impl ContextMenu {
    pub fn new(items: Menu) -> Self {
        ContextMenu {focused: None, items: items.into_iter().map(From::from).collect()}
    }

    fn focused(&mut self) -> Option<&mut ContextMenuItem> {
        self.focused.map(move |index| self.items.index_mut(index))
    }

    fn focus_prev(&mut self, core: &Core) {
        let index = self.unfocus();
        self.focused = self.items
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(index.map(|i| self.items.len() - i).unwrap_or(0))
            .filter(|&(_, ref item)| item.enabled(core))
            .next()
            .map(|(i, _)| i);
    }

    fn focus_next(&mut self, core: &Core) {
        let index = self.unfocus();
        self.focused = self.items
            .iter()
            .enumerate()
            .cycle()
            .skip(index.map(|i| i + 1).unwrap_or(0))
            .filter(|&(_, ref item)| item.enabled(core))
            .next()
            .map(|(i, _)| i);
    }

    pub fn unfocus(&mut self) -> Option<usize> {
        if let Some(index) = self.focused {
            if let ContextMenuItem::Group(ref mut group) = self.items[index] {
                group.is_opened = false;
                group.modal.content.unfocus();
                group.modal.hide();
            }
            self.focused = None;
            Some(index)
        } else {
            None
        }
    }
}

impl<'a> Widget<'a> for ContextMenu {
    type Context = Core;
    type View = ContextMenuView<'a>;

    fn view(&'a self, core: &Core) -> ContextMenuView<'a> {
        let views = self.items.iter().enumerate().map(|(i, item)|
            match *item {
                ContextMenuItem::Divider => ContextMenuItemView::Divider,
                ContextMenuItem::Button(ref button) =>
                    ContextMenuItemView::Button(button.view(core, self.focused == Some(i))),
                ContextMenuItem::Group(ref group) =>
                    ContextMenuItemView::Group(group.view(core, self.focused == Some(i)))
            }
        ).collect::<Vec<_>>();

        ContextMenuView { views: views }
    }

    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        let processed = match self.focused() {
            Some(&mut ContextMenuItem::Group(ref mut group)) if group.is_opened =>
                group.modal.on_keypress(core, canvas, key),
            _ => false
        };
        if processed {
            return true;
        }
        match key {
            Key::Up => self.focus_prev(core),
            Key::Down => self.focus_next(core),
            Key::Right => match self.focused() {
                Some(&mut ContextMenuItem::Group(ref mut group)) if !group.is_opened => {
                    group.is_opened = true;
                    group.modal.content.focus_next(core);
                },
                _ => return false
            },
            Key::Left => match self.focused() {
                Some(&mut ContextMenuItem::Group(ref mut group)) if group.is_opened => {
                    group.is_opened = false;
                    group.modal.content.unfocus();
                    group.modal.hide();
                },
                _ => return false
            },
            _ => return false
        }
        self.view(core).render(canvas);
        return true;
    }
}
