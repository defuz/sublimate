use core::Core;
use view::modal::{Modal, ModalPosition};
use core::bindings::Key;
use core::menu::{Menu};

use toolkit::*;

use unicode_width::UnicodeWidthStr;

use view::modal::ModalView;

use view::theme::*;

use super::menu::{ContextMenu, ContextMenuView};

#[derive(Debug)]
pub struct Group {
    caption: String,
    modal: Modal<ContextMenu>,
    is_opened: bool
}

#[derive(Debug)]
pub struct GroupView<'a> {
    pub caption: &'a str,
    pub is_focused: bool,
    pub modal: Option<ModalView<'a, ContextMenuView<'a>>>,
}

impl Group {
    pub fn new(caption: String, menu: Menu) -> Group {
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
        false
    }

    fn focus(&mut self, (core, _): Self::Context) {
        self.is_opened = true;
        self.modal.focus(core);
    }

    fn unfocus(&mut self, (core, _): Self::Context) {
        self.is_opened = false;
        self.modal.unfocus(core);
    }
}

impl<'a> GroupView<'a> {
    fn caption_style(&self) -> Style {
        if self.is_focused {
            MODAL_SELECTED_STYLE
        } else {
            MODAL_STYLE
        }
    }

    fn arrow_style(&self) -> Style {
        if self.is_focused {
            MODAL_SELECTED_LOW_STYLE
        } else {
            MODAL_LOW_STYLE
        }
    }
}

impl<'a> View for GroupView<'a> {
    fn width(&self) -> usize {
        self.caption.width() + 5
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, mut canvas: Canvas) {
        if let Some(ref view) = self.modal {
            view.render(canvas.clone());
        }
        canvas.style(self.caption_style());
        let left_canvas = canvas.cut_left(self.caption.width() + 1);
        let right_canvas = canvas.cut_right(2);
        left_canvas.char(' ', 0, 0);
        left_canvas.text(self.caption, 0, 1);
        canvas.fill();
        right_canvas.style(self.arrow_style());
        right_canvas.char('▸', 0, 0);
        right_canvas.char(' ', 0, 1);
    }
}

