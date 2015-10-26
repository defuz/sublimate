use toolkit::*;
use core::Core;
use core::keymap::Key;
use core::menu::{Menu, MenuItem};
use core::command::Command;
use view::theme::*;

use view::event::{OnKeypress, OnKeypressComponent};
use view::context::ContextMenu;

#[derive(Debug)]
pub struct Menubar {
    focused: Option<usize>,
    items: Box<[MenubarItem]>
}

#[derive(Debug)]
pub struct MenubarItem {
    pub id: Box<str>,
    pub name: Box<str>
}

impl<'c> View<&'c Core> for MenubarItem {
    fn width(&self, core: &Core) -> usize {
        self.name.len() + 2
    }

    fn height(&self, core: &Core) -> usize {
        1
    }

    fn render(&self, core: &Core, canvas: Canvas) {
        // info!("render {}, {:?}", self.name, canvas)
        canvas.char(' ', 0, 0);
        canvas.text(&*self.name, 0, 1);
        canvas.char(' ', 0, self.name.len() + 1);
    }
}

impl Menubar {
    pub fn new(core: &Core) -> (Menubar, Vec<ContextMenu>) {
        let mut items = Vec::new();
        let mut menus = Vec::new();
        for item in core.package_repository.get_menu("default/Main.sublime-menu") {
            match item {
                MenuItem::Group(name, menu) => {
                    items.push(MenubarItem {
                        id: "id".to_string().into_boxed_str(),
                        name: name.clone().into_boxed_str()
                    });
                    menus.push(ContextMenu::new(menu))
                },
                _ => {
                    error!("Incorrect menu item")
                }
            }
        }
        (Menubar {
            focused: Some(3),
            items: items.into_boxed_slice()
        }, menus)
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

impl<'c> View<&'c Core> for Menubar {
    fn width(&self, core: &Core) -> usize {
        sum_width(core, self.items.iter())
    }

    fn height(&self, core: &Core) -> usize {
        1
    }

    fn render(&self, core: &Core, mut canvas: Canvas) {
        MENUBAR_STYLE.set();
        for (i, item) in self.items.iter().enumerate() {
            let w = item.width(core);
            if w > canvas.width() {
                break;
            }
            let item_canvas = canvas.cut_left(w);
            if self.focused == Some(i) {
                MENUBAR_SELECTED_STYLE.set();
                item.render(core, item_canvas);
                MENUBAR_STYLE.set();
            } else {
                item.render(core, item_canvas);
            }
        }
        canvas.fill();
    }
}

impl<'c> OnKeypress<&'c Core> for MenubarItem {
    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        // core.get_context(self.id).on_keypress(core, key)
        match key {
            Key::Up | Key::Down => true,
            _ => false
        }
    }
}

impl<'c> OnKeypressComponent<&'c Core> for Menubar {
    type T = MenubarItem;

    fn focused(&mut self, core: &Core, canvas: Canvas) -> Option<(&mut Self::T, Canvas)> {
        match self.focused {
            Some(index) => Some((&mut self.items[index], canvas)),
            None => None
        }
    }

    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        match key {
            Key::Left => {
                self.focus_prev();
                self.render(core, canvas);
                true
            },
            Key::Right => {
                self.focus_next();
                self.render(core, canvas);
                true
            },
            _ => false
        }
    }
}
