use std::ops::IndexMut;

use unicode_width::UnicodeWidthStr;

use toolkit::*;
use core::Core;
use core::keymap::Key;
use core::menu::MenuItem;
use view::theme::*;

use view::context::ContextMenu;
use view::context::view::ContextMenuView;
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
    pub is_focused: bool,
    pub modal: ModalView<'a, ContextMenuView<'a>>
}

pub struct MenubarView<'a> {
    views: Vec<MenubarItemView<'a>>
}

impl Menubar {
    pub fn new(core: &Core) -> Menubar {
        let mut items = Vec::new();
        for item in core.package_repository.get_menu("default/Main.sublime-menu") {
            match item {
                MenuItem::Group(caption, menu) => {
                    items.push(MenubarItem {
                        caption: caption.clone(),
                        modal: Modal::new(ContextMenu::new(menu), ModalPosition::UnderLeft)
                    });
                },
                _ => error!("Incorrect menu item")
            }
        }
        Menubar {focused: None, items: items}
    }

    fn focused(&mut self, core: &Core, mut canvas: Canvas) -> Option<&mut MenubarItem> {
        match self.focused {
            Some(index) => {
                Some(self.items.index_mut(index))
            },
            None => None
        }
    }

    fn focus_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.focused = Some(match self.focused {
            Some(index) => {
                self.items[index].modal.content.unfocus();
                self.items[index].modal.hide();
                (index + self.items.len() - 1) % self.items.len()
            },
            _ => self.items.len() - 1
        })
    }

    fn focus_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.focused = Some(match self.focused {
            Some(index) => {
                self.items[index].modal.content.unfocus();
                self.items[index].modal.hide();
                (index + 1) % self.items.len()
            },
            None => 0
        })
    }

}

impl<'a> Widget<'a> for Menubar {
    type Context = Core;
    type View = MenubarView<'a>;

    fn view(&'a self, core: &Core) -> MenubarView<'a> {
        let views = self.items.iter().enumerate().map(|(i, item)| MenubarItemView {
            caption: &item.caption,
            is_focused: Some(i) == self.focused,
            modal: item.modal.view(core)
        }).collect();
        MenubarView { views: views }
    }

    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        if let Some(child) = self.focused(core, canvas) {
            if child.modal.on_keypress(core, canvas, key) {
                return true;
            }
        }
        match key {
            Key::Left => self.focus_prev(),
            Key::Right => self.focus_next(),
            _ => return false
        }
        self.view(core).render(canvas);
        return true;
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
        canvas.style(if self.is_focused {
            MENUBAR_SELECTED_STYLE
        } else {
            MENUBAR_STYLE
        });
        canvas.char(' ', 0, 0);
        canvas.text(self.caption, 0, 1);
        canvas.char(' ', 0, self.caption.width() + 1);
        if self.is_focused {
            self.modal.render(canvas)
        }
    }
}

impl<'a> View for MenubarView<'a> {
    fn width(&self) -> usize {
        let mut r = 0;
        for v in self.views.iter() {
            r += v.width();
        }
        return r;
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, mut canvas: Canvas) {
        canvas.style(MENUBAR_STYLE);
        for (i, item) in self.views.iter().enumerate() {
            let w = item.width();
            if w > canvas.width() {
                break;
            }
            item.render(canvas.cut_left(w))
        }
        canvas.fill();
    }
}
