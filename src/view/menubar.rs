use toolkit::*;
use core::Core;
use core::keymap::Key;
use core::menu::{Menu, MenuItem};
use core::command::Command;
use view::theme::*;

use view::window::{Window, Context};
use view::event::OnKeypress;
use view::context::ContextMenu;
use view::modal::{Modal, ModalPosition};

#[derive(Debug)]
pub struct Menubar {
    focused: Option<usize>,
    items: Vec<MenubarItem>
}

#[derive(Debug)]
pub struct MenubarItem {
    pub name: String,
    pub items: Modal<Core, ContextMenu>
}

impl View<Core> for MenubarItem {
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
                        name: name.clone(),
                        items: Modal::new(ContextMenu::new(menu), ModalPosition::UnderLeft)
                    });
                },
                _ => {
                    error!("Incorrect menu item")
                }
            }
        }
        (Menubar {
            focused: Some(3),
            items: items
        }, menus)
    }

    fn focused(&mut self, core: &Core, mut canvas: Canvas) -> Option<(&MenubarItem, Canvas)> {
        match self.focused {
            Some(index) => {
                // FIXME: if canvas is less then needed, return None instead of item canvas
                for item in self.items.iter().take(index) {
                    canvas.cut_left(item.width(core));
                }
                let ref item = self.items[index];
                let item_canvas = canvas.cut_left(item.width(core));
                Some((item, item_canvas))
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
                self.items[index].items.hide();
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
                self.items[index].items.hide();
                (index + 1) % self.items.len()
            },
            None => 0
        })
    }

}

impl View<Core> for Menubar {
    fn width(&self, core: &Core) -> usize {
        sum_width(core, self.items.iter())
    }

    fn height(&self, core: &Core) -> usize {
        1
    }

    fn render(&self, core: &Core, mut canvas: Canvas) {
        canvas.style(MENUBAR_STYLE);
        for (i, item) in self.items.iter().enumerate() {
            let w = item.width(core);
            if w > canvas.width() {
                break;
            }
            let item_canvas = canvas.cut_left(w);
            if self.focused == Some(i) {
                item_canvas.style(MENUBAR_SELECTED_STYLE);
                item.render(core, item_canvas);
                item.items.render(core, item_canvas);
                item_canvas.style(MENUBAR_STYLE);
            } else {
                item.render(core, item_canvas);
            }
        }
        canvas.fill();
    }
}

// impl<'c> OnKeypress<Context<'c>> for MenubarItem {
//     fn on_keypress(&mut self, context: Context<'c>, canvas: Canvas, key: Key) -> bool {
//         // context.models.get_modal(self.id).on_keypress(context, key)
//         match key {
//             Key::Up | Key::Down => true,
//             _ => false
//         }
//     }
// }

impl<'c> OnKeypress<Context<'c>> for Menubar {

    fn on_keypress(&mut self, context: Context<'c>, canvas: Canvas, key: Key) -> bool {
        // if let Some((child, canvas)) = self.focused(canvas) {
        //     if child.on_keypress(context, canvas, key) {
        //         return true;
        //     }
        // }
        match key {
            Key::Left => self.focus_prev(),
            Key::Right => self.focus_next(),
            _ => return false
        }
        // if let Some((item, c)) = self.focused(context.core, canvas) {
            // context.modals.replace_modal_window(item.id, context.core, ModalPosition::UnderLeft(c))
        // }
        self.render(context.core, canvas);
        return true;
    }
}
