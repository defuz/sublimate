use unicode_width::UnicodeWidthStr;

use toolkit::*;
use core::Core;
use core::bindings::Key;
use core::menu::MenuItem;
use view::theme::*;

use view::context::{ContextMenu, ContextMenuView};
use view::modal::{Modal, ModalView, ModalPosition};

#[derive(Debug)]
pub struct Menubar {
    focused: Option<usize>,
    items: Vec<MenubarItem>
}

#[derive(Debug)]
pub struct MenubarItem {
    pub caption: String,
    pub modal: Modal<ContextMenu>
}

#[derive(Debug)]
pub struct MenubarItemView<'a> {
    pub caption: &'a str,
    pub modal: Option<ModalView<'a, ContextMenuView<'a>>>
}

pub struct MenubarView<'a> {
    views: Vec<MenubarItemView<'a>>
}

impl Menubar {
    pub fn new(core: &Core) -> Menubar {
        let mut items = Vec::new();
        for item in core.create_menu() {
            match item {
                MenuItem::Group(caption, menu) => {
                    items.push(MenubarItem {
                        caption: caption,
                        modal: Modal::new(ContextMenu::new(menu), ModalPosition::UnderLeft)
                    });
                },
                _ => error!("Incorrect menu item")
            }
        }
        Menubar {focused: None, items: items}
    }

    fn focused(&mut self) -> Option<&mut MenubarItem> {
        match self.focused {
            Some(index) => {
                Some(&mut self.items[index])
            },
            None => None
        }
    }

    fn focus_prev(&mut self, core: &Core) {
        if self.items.is_empty() {
            return
        }
        self.focused = Some(match self.focused {
            Some(index) => {
                self.items[index].modal.unfocus(core);
                (index + self.items.len() - 1) % self.items.len()
            },
            _ => self.items.len() - 1
        })
    }

    fn focus_next(&mut self, core: &Core) {
        if self.items.is_empty() {
            return
        }
        self.focused = Some(match self.focused {
            Some(index) => {
                self.items[index].modal.unfocus(core);
                (index + 1) % self.items.len()
            },
            None => 0
        })
    }

}

impl<'a> Widget<'a> for Menubar {
    type Context = &'a Core;
    type View = MenubarView<'a>;

    fn view(&'a self, core: &'a Core) -> MenubarView<'a> {
        let views = self.items.iter().enumerate().map(|(i, item)| MenubarItemView {
            caption: &item.caption,
            modal: if Some(i) == self.focused {
                Some(item.modal.view(core))
            } else {
                None
            }
        }).collect();
        MenubarView { views: views }
    }

    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        if let Some(child) = self.focused() {
            if child.modal.on_keypress(core, canvas, key) {
                return true
            }
        }
        match key {
            Key::Left => self.focus_prev(core),
            Key::Right => self.focus_next(core),
            _ => return false
        }
        self.view(core).render(canvas);
        return true;
    }
}

impl<'a> MenubarItemView<'a> {
    fn is_focused(&self) -> bool {
        self.modal.is_some()
    }

    fn style(&self) -> Style {
        if self.is_focused() {
            MENUBAR_SELECTED_STYLE
        } else {
            MENUBAR_STYLE
        }
    }
}

impl<'a> View for MenubarItemView<'a> {
    fn width(&self) -> usize {
        self.caption.width() + 2
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, canvas: Canvas) {
        canvas.style(self.style());
        canvas.char(' ', 0, 0);
        canvas.text(self.caption, 0, 1);
        canvas.char(' ', 0, self.caption.width() + 1);
        if let Some(ref modal) = self.modal {
            modal.render(canvas)
        }
    }
}

impl<'a> View for MenubarView<'a> {
    fn width(&self) -> usize {
        let mut r = 0;
        for v in self.views.iter() {
            r += v.width();
        }
        return r
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, mut canvas: Canvas) {
        for view in self.views.iter() {
            let w = view.width();
            if w > canvas.width() {
                break
            }
            view.render(canvas.cut_left(w))
        }
        canvas.style(MENUBAR_STYLE);
        canvas.fill();
    }
}
