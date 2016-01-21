use core::Core;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::bindings::Key;
use core::menu::{Menu, MenuItem};

use toolkit::*;

use unicode_width::UnicodeWidthStr;

use view::modal::ModalView;

use view::theme::*;

use super::button::{Button, ButtonView};
use super::group::{Group, GroupView};

#[derive(Debug)]
pub enum ContextMenuItem {
    Divider,
    Button(Button),
    Group(Group),
}

#[derive(Debug)]
pub enum ContextMenuItemView<'a> {
    Divider,
    Button(ButtonView<'a>),
    Group(GroupView<'a>),
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

impl<'a> View for ContextMenuItemView<'a> {
    fn width(&self) -> usize {
        match *self {
            ContextMenuItemView::Divider => 2,
            ContextMenuItemView::Group(ref view) => view.width(),
            ContextMenuItemView::Button(ref view) => view.width()
        }
    }

    fn height(&self) -> usize {
        match *self {
            ContextMenuItemView::Divider => 1,
            ContextMenuItemView::Group(ref view) => view.height(),
            ContextMenuItemView::Button(ref view) => view.height()
        }
    }

    fn render(&self, canvas: Canvas) {
        match *self {
            ContextMenuItemView::Divider => {
                canvas.style(MODAL_DISABLED_STYLE);
                canvas.fill_char('─');
            },
            ContextMenuItemView::Group(ref view) => view.render(canvas),
            ContextMenuItemView::Button(ref view) => view.render(canvas)
        }
    }
}
