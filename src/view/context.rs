use unicode_width::UnicodeWidthStr;

use core::Core;
use core::keymap::{Hotkey, HotkeySequence};
use core::menu::{Menu, MenuItem};

use toolkit::*;
use view::theme::*;

fn hotkey_to_string(key: Option<&HotkeySequence>) -> String {
    "Ctrl+Alt+A".to_string()
}

#[derive(Debug)]
pub struct ContextMenu {
    focused: Option<usize>,
    menu: Menu
}

impl ContextMenu {
    pub fn new(menu: Menu) -> Self {
        ContextMenu {focused: None, menu: menu}
    }
}

impl<'c> View<&'c Core> for ContextMenu {
    fn width(&self, core: &Core) -> usize {
        self.menu.iter().map(|v| v.width((false, core))).max().unwrap_or(0)
    }

    fn height(&self, core: &Core) -> usize {
        self.menu.len()
    }

    fn render(&self, core: &Core, mut canvas: Canvas) {
        for (i, item) in self.menu.iter().enumerate() {
            let h = item.height((false, core));
            if h > canvas.height() {
                break;
            }
            let item_canvas = canvas.cut_top(h);
            let context = (self.focused == Some(i), core);
            item.render(context, item_canvas);
        }
    }
}


impl<'c> View<(bool, &'c Core)> for MenuItem {
    fn width(&self, (focused, core): (bool, &Core)) -> usize {
        match *self {
            MenuItem::Divider => 2,
            MenuItem::Group(ref caption, _) => caption.width() + 5,
            MenuItem::Button(ref caption, ref command, _) => {
                let caption: &str = match *caption {
                    Some(ref c) => c,
                    None => "<No caption>"
                };
                let hotkey = hotkey_to_string(core.hotkeys.get_hotkeys(&command));
                caption.width() + hotkey.width() + 4
            }
        }
    }

    fn height(&self, (focused, core): (bool, &Core)) -> usize {
        1
    }

    fn render(&self, (focused, core): (bool, &Core), mut canvas: Canvas) {
        match *self {
            MenuItem::Divider => {
                MODAL_DISABLED_STYLE.set();
                canvas.fill_char('─');
            },
            MenuItem::Group(ref caption, _) => {
                let (style, low_style) = if focused {
                    (MODAL_SELECTED_STYLE, MODAL_SELECTED_LOW_STYLE)
                } else {
                    (MODAL_STYLE, MODAL_LOW_STYLE)
                };
                let left_canvas = canvas.cut_left(caption.width() + 1);
                let right_canvas = canvas.cut_right(2);
                style.set();
                left_canvas.char(' ', 0, 0);
                left_canvas.text(caption, 0, 1);
                canvas.fill();
                low_style.set();
                right_canvas.char('▸', 0, 0);
                right_canvas.char(' ', 0, 1);
            },
            MenuItem::Button(ref caption, ref command, _) => {
                let enabled = true;
                let caption: &str = match *caption {
                    Some(ref c) => c,
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
                let left_canvas = canvas.cut_left(caption.width() + 1);
                let right_canvas = canvas.cut_right(hotkey.width() + 1);
                style.set();
                left_canvas.char(' ', 0, 0);
                left_canvas.text(caption, 0, 1);
                canvas.fill();
                low_style.set();
                right_canvas.text(&hotkey, 0, 0);
                right_canvas.char(' ', 0, hotkey.width());
            }
        }
    }
}
