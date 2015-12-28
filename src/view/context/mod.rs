use core::Core;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::bindings::Key;
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
}

impl<'a> Widget<'a> for Button {
    type Context = (&'a Core, bool);
    type View = ButtonView<'a>;

    fn view(&'a self, (core, focused): Self::Context) -> ButtonView<'a> {
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
}

impl<'a> Widget<'a> for Group {
    type Context = (&'a Core, bool);
    type View = GroupView<'a>;

    fn view(&'a self, (core, focused): Self::Context) -> GroupView<'a> {
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

    fn on_keypress(&'a mut self, (core, focused): Self::Context, canvas: Canvas, key: Key) -> bool {
        if self.is_opened {
            if self.modal.on_keypress(core, canvas, key) {
                return true
            }
            if key == Key::Left {
                self.unfocus((core, focused));
                return true
            }
        } else {
            if key == Key::Right {
                self.focus((core, focused));
                return true
            }
        }
        return false;
    }

    fn focus(&mut self, (core, focused): Self::Context) {
        self.is_opened = true;
        self.modal.focus(core);
    }

    fn unfocus(&mut self, (core, focused): Self::Context) {
        self.is_opened = false;
        self.modal.unfocus(core);
    }
}

impl<'a> Widget<'a> for ContextMenuItem {
    type Context = (&'a Core, bool);
    type View = ContextMenuItemView<'a>;

    fn enabled(&self, context: Self::Context) -> bool {
        match *self {
            ContextMenuItem::Button(ref button) => button.enabled(context),
            ContextMenuItem::Group(ref group) => group.enabled(context),
            ContextMenuItem::Divider => false
        }
    }

    fn view(&'a self, context: Self::Context) -> Self::View {
        match *self {
            ContextMenuItem::Button(ref button) =>
                ContextMenuItemView::Button(button.view(context)),
            ContextMenuItem::Group(ref group) =>
                ContextMenuItemView::Group(group.view(context)),
            ContextMenuItem::Divider => ContextMenuItemView::Divider
        }
    }

    fn on_keypress(&'a mut self, context: (&'a Core, bool), canvas: Canvas, key: Key) -> bool {
        match *self {
            ContextMenuItem::Button(ref mut button) => button.on_keypress(context, canvas, key),
            ContextMenuItem::Group(ref mut group) => group.on_keypress(context, canvas, key),
            ContextMenuItem::Divider => false
        }
    }

    fn unfocus(&mut self, context: Self::Context) {
        match *self {
            ContextMenuItem::Button(ref mut button) => button.unfocus(context),
            ContextMenuItem::Group(ref mut group) => group.unfocus(context),
            ContextMenuItem::Divider => {},
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
