use toolkit::*;
use core::Core;
use core::menu::{Menu, MenuItem};
use core::command::Command;
use view::theme::*;

const MENUBAR_STYLE          : Style = Style {
    colors: MENUBAR_COLORS,
    attrs: NORMAL,
};
const MENUBAR_SELECTED_STYLE : Style = Style {
    colors: MENUBAR_SELECTED_COLORS,
    attrs: NORMAL,
};

pub struct MenubarItem {
    pub id: Box<str>,
    pub name: Box<str>,
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


pub struct Menubar {
    focused: Option<usize>,
    items: Box<[MenubarItem]>,
}

impl Menubar {
    pub fn new(core: &Core) -> Menubar {
        let mut items = Vec::new();
        for item in core.package_repository.get_menu("default/Main.sublime-menu").iter() {
            match item {
                &MenuItem::Group(ref name, _) => items.push(MenubarItem {
                    id: "id".to_string().into_boxed_str(),
                    name: name.clone().into_boxed_str(),
                }),
                _ => {
                    error!("Incorrect menu item")
                }
            }
        }
        Menubar {
            focused: Some(3),
            items: items.into_boxed_slice(),
        }
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
    }
}
