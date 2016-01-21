use core::Core;
use view::modal::{Modal, ModalPosition};
use core::command::Command;
use core::bindings::Key;
use core::menu::{Menu, MenuItem};

use toolkit::*;

use unicode_width::UnicodeWidthStr;

use view::modal::ModalView;

use view::theme::*;

#[derive(Debug)]
pub struct Button {
    caption: Option<String>,
    command: Command,
    is_checkbox: bool
}

#[derive(Debug)]
pub struct ButtonView<'a> {
    pub caption: &'a str,
    pub hotkey: String,
    pub state: ButtonState
}

#[derive(Debug)]
pub enum ButtonState {
    Unfocused,
    Focused,
    Disabled
}

impl Button {
    pub fn new(caption: Option<String>, command: Command, is_checkbox: bool) -> Button {
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

impl ButtonState {
    fn caption_style(&self) -> Style {
        match *self {
            ButtonState::Unfocused => MODAL_STYLE,
            ButtonState::Focused => MODAL_SELECTED_STYLE,
            ButtonState::Disabled => MODAL_DISABLED_STYLE
        }
    }

    fn hotkey_style(&self) -> Style {
        match *self {
            ButtonState::Unfocused => MODAL_LOW_STYLE,
            ButtonState::Focused => MODAL_SELECTED_LOW_STYLE,
            ButtonState::Disabled => MODAL_DISABLED_STYLE
        }
    }
}

impl<'a> View for ButtonView<'a> {
    fn width(&self) -> usize {
        self.caption.width() + self.hotkey.width() + 4
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, mut canvas: Canvas) {
        canvas.style(self.state.caption_style());
        let left_canvas = canvas.cut_left(self.caption.width() + 1);
        let right_canvas = canvas.cut_right(self.hotkey.width() + 1);
        left_canvas.char(' ', 0, 0);
        left_canvas.text(self.caption, 0, 1);
        canvas.fill();
        right_canvas.style(self.state.hotkey_style());
        right_canvas.text(&self.hotkey, 0, 0);
        right_canvas.char(' ', 0, self.hotkey.width());
    }
}
