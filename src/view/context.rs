use std::io::Write;
use unicode_width::UnicodeWidthStr;

use core::Core;
use view::window::Window;
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
    items: Menu
}

#[derive(Debug)]
struct MenuItemView<'a>(&'a MenuItem, /* is selected? */ bool);

impl ContextMenu {
    pub fn new(items: Menu) -> Self {
        ContextMenu {focused: None, items: items}
    }

    fn focus_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.focused = Some(match self.focused {
            Some(index) if index != 0 => index - 1,
            _ => self.items.len() - 1
        })
    }

    fn focus_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.focused = Some(match self.focused {
            Some(index) => (index + 1) % self.items.len(),
            None => 0
        })
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
            MenuItem::Divider => 2,
            MenuItem::Group(ref caption, _) => caption.width() + 5,
            MenuItem::Button(ref caption, ref command, _) => {
                let caption = match *caption {
                    Some(ref c) => c.as_str(),
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
            MenuItem::Divider => {
                canvas.style(MODAL_DISABLED_STYLE);
                canvas.fill_char('─');
            },
            MenuItem::Group(ref caption, _) => {
                let (style, low_style) = if focused {
                    (MODAL_SELECTED_STYLE, MODAL_SELECTED_LOW_STYLE)
                } else {
                    (MODAL_STYLE, MODAL_LOW_STYLE)
                };
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
            MenuItem::Button(ref caption, ref command, _) => {
                let enabled = true;
                let caption = match *caption {
                    Some(ref c) => c.as_str(),
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
        match key {
            Key::Up => self.focus_prev(),
            Key::Down => self.focus_next(),
            _ => return false
        }
        // if let Some((item, c)) = self.focused(context.core, canvas) {
        //     context.modals.replace_modal_window(item.id, context.core, ModalPosition::UnderLeft(c))
        // }
        self.render(core, canvas);
        return true;
    }
}

