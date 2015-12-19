use std::io::Write;
use std::ops::IndexMut;
use unicode_width::UnicodeWidthStr;

use core::Core;
use view::window::Window;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::keymap::{Key, Hotkey, HotkeySequence};
use core::menu::{Menu, MenuItem};

use toolkit::*;
use view::theme::*;
use view::event::OnKeypress;

fn hotkey_to_string(keys: Option<&HotkeySequence>) -> String {
    match keys {
        Some(keys) => {
            let mut buf = Vec::new();
            let mut comma = false;
            for key in keys {
                if comma {
                    write!(buf, ", ");
                }
                comma = true;
                write!(buf, "{}", key);
            }
            String::from_utf8(buf).unwrap()
        },
        None => "".to_string()
    }
}

#[derive(Debug)]
pub struct ContextMenu {
    focused: Option<usize>,
    items: Vec<ContextMenuItem>
}

#[derive(Debug)]
pub enum ContextMenuItem {
    Button(Option<String>, Command, bool),
    Group(String, /* opened? */ bool, Modal<Core, ContextMenu>),
    Divider,
}

impl ContextMenuItem {
    fn enabled(&self, core: &Core) -> bool {
        match *self {
            ContextMenuItem::Button(..) | ContextMenuItem::Group(..) => true,
            ContextMenuItem::Divider => false
        }
    }
}

#[derive(Debug)]
struct MenuItemView<'a>(&'a ContextMenuItem, /* is selected? */ bool);

impl From<MenuItem> for ContextMenuItem {
    fn from(item: MenuItem) -> ContextMenuItem {
        match item {
            MenuItem::Button(name, command, is_checkbox) =>
                ContextMenuItem::Button(name, command, is_checkbox),
            MenuItem::Group(name, menu) => {
                let modal = Modal::new(ContextMenu::new(menu), ModalPosition::RightTop);
                ContextMenuItem::Group(name, false, modal)
            },
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
            if let ContextMenuItem::Group(_, ref mut opened, ref mut modal) = self.items[index] {
                *opened = false;
                modal.content.unfocus();
                modal.hide();
            }
            self.focused = None;
            Some(index)
        } else {
            None
        }
    }
}

impl View<Core> for ContextMenu {

    fn width(&self, core: &Core) -> usize {
        self.items.iter().map(|i| MenuItemView(i, false).width(core)).max().unwrap_or(0)
    }

    fn height(&self, core: &Core) -> usize {
        self.items.len()
    }

    fn render(&self, core: &Core, mut canvas: Canvas) {
        for (i, item) in self.items.iter().enumerate() {
            let view = MenuItemView(item, self.focused == Some(i));
            let h = view.height(core);
            if h > canvas.height() {
                break;
            }
            let item_canvas = canvas.cut_top(h);
            view.render(core, item_canvas);
        }
    }
}


impl<'c> View<Core> for MenuItemView<'c> {
    fn width(&self, core: &Core) -> usize {
        let MenuItemView(item, _) = *self;
        match *item {
            ContextMenuItem::Divider => 2,
            ContextMenuItem::Group(ref caption, _, _) => caption.width() + 5,
            ContextMenuItem::Button(ref caption, ref command, _) => {
                let caption = match *caption {
                    Some(ref c) => &c[..],
                    None => "<No caption>"
                };
                let hotkey = hotkey_to_string(core.hotkeys.get_hotkeys(&command));
                caption.width() + hotkey.width() + 4
            }
        }
    }

    fn height(&self, core: &Core) -> usize {
        1
    }

    fn render(&self, core: &Core, mut canvas: Canvas) {
        let MenuItemView(item, focused) = *self;
        match *item {
            ContextMenuItem::Divider => {
                canvas.style(MODAL_DISABLED_STYLE);
                canvas.fill_char('─');
            },
            ContextMenuItem::Group(ref caption, opened, ref modal) => {
                let (style, low_style) = if focused {
                    (MODAL_SELECTED_STYLE, MODAL_SELECTED_LOW_STYLE)
                } else {
                    (MODAL_STYLE, MODAL_LOW_STYLE)
                };
                if opened {
                    modal.render(core, canvas.clone());
                }
                canvas.style(style);
                let left_canvas = canvas.cut_left(caption.width() + 1);
                let right_canvas = canvas.cut_right(2);
                left_canvas.char(' ', 0, 0);
                left_canvas.text(caption, 0, 1);
                canvas.fill();
                right_canvas.style(low_style);
                right_canvas.char('▸', 0, 0);
                right_canvas.char(' ', 0, 1);
            },
            ContextMenuItem::Button(ref caption, ref command, _) => {
                let enabled = true;
                let caption = match *caption {
                    Some(ref c) => &c[..],
                    None => "<No caption>"
                };
                let hotkey = hotkey_to_string(core.hotkeys.get_hotkeys(&command));
                let (style, low_style) = if focused {
                    (MODAL_SELECTED_STYLE, MODAL_SELECTED_LOW_STYLE)
                } else if enabled {
                    (MODAL_STYLE, MODAL_LOW_STYLE)
                } else {
                    (MODAL_DISABLED_STYLE, MODAL_DISABLED_STYLE)
                };
                canvas.style(style);
                let left_canvas = canvas.cut_left(caption.width() + 1);
                let right_canvas = canvas.cut_right(hotkey.width() + 1);
                left_canvas.char(' ', 0, 0);
                left_canvas.text(caption, 0, 1);
                canvas.fill();
                right_canvas.style(low_style);
                right_canvas.text(&hotkey, 0, 0);
                right_canvas.char(' ', 0, hotkey.width());
            }
        }
    }
}

impl OnKeypress<Core> for ContextMenu {
    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        let processed = match self.focused() {
            Some(&mut ContextMenuItem::Group(_, true, ref mut modal)) =>
                modal.on_keypress(core, canvas, key),
            _ => false
        };
        if processed {
            return true;
        }
        match key {
            Key::Up => self.focus_prev(core),
            Key::Down => self.focus_next(core),
            Key::Right => match self.focused() {
                Some(&mut ContextMenuItem::Group(_, ref mut opened, ref mut modal)) if !*opened => {
                    *opened = true;
                    modal.content.focus_next(core);
                },
                _ => return false
            },
            Key::Left => match self.focused() {
                Some(&mut ContextMenuItem::Group(_, ref mut opened, ref mut modal)) if *opened => {
                    *opened = false;
                    modal.content.unfocus();
                    modal.hide();
                },
                _ => return false
            },
            _ => return false
        }
        self.render(core, canvas);
        return true;
    }
}

