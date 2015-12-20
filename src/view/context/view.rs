use unicode_width::UnicodeWidthStr;

use view::modal::ModalView;

use toolkit::*;
use view::theme::*;

#[derive(Debug)]
pub struct ContextMenuView<'a> {
    pub views: Vec<ContextMenuItemView<'a>>
}

#[derive(Debug)]
pub enum ContextMenuItemView<'a> {
    Divider,
    Button(ButtonView<'a>),
    Group(GroupView<'a>),
}

#[derive(Debug)]
pub struct GroupView<'a> {
    pub caption: &'a str,
    pub is_focused: bool,
    pub modal: Option<ModalView<'a, ContextMenuView<'a>>>,
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

