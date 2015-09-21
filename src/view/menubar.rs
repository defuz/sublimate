use toolkit::*;
use core::menu::{Menu, MenuItem};
use core::command::Command;
use view::theme::*;

pub type MenuBar = Decorator<HorizontalWidget<MenuGroup>>;
pub type MenuBox = VerticalWidget<MenuChild>;

#[derive(Debug)]
struct MenuGroup {
    caption: String,
    submenu: MenuBox
}

#[derive(Debug)]
enum MenuChild {
    Button(String, Command, bool),
    Group(String, MenuBox),
    Divider
}

impl Menu for MenuBar {
    type I = MenuGroup;

    fn from_vec(items: Vec<MenuGroup>) -> MenuBar {
        MenuBar {
            style: Style {
                colors: MENUBAR,
                attrs: NORMAL
            },
            item: HorizontalWidget {
                children: items
            }
        }
    }
}

impl MenuItem for MenuGroup {
    type M = MenuBox;

    fn divider() -> Option<MenuGroup> {
        None
    }

    fn button(caption: String, command: Command, is_checkbox: bool) -> Option<MenuGroup> {
        None
    }

    fn group(caption: String, submenu: MenuBox) -> Option<MenuGroup> {
        Some(MenuGroup { caption: caption, submenu: submenu })
    }
}

impl Menu for MenuBox {
    type I = MenuChild;

    fn from_vec(items: Vec<MenuChild>) -> MenuBox {
        MenuBox {children: items}
    }
}

impl MenuItem for MenuChild {
    type M = MenuBox;

    fn divider() -> Option<MenuChild> {
        Some(MenuChild::Divider)
    }

    fn button(caption: String, command: Command, is_checkbox: bool) -> Option<MenuChild> {
        Some(MenuChild::Button(caption, command, is_checkbox))
    }

    fn group(caption: String, submenu: MenuBox) -> Option<MenuChild> {
        Some(MenuChild::Group(caption, submenu))
    }
}

impl HasWidth for MenuGroup {
    fn width(&self) -> size {
        self.caption.len() as u16 + 2
    }
}

impl HasHeight for MenuGroup {
    fn height(&self) -> size {
        1
    }
}

impl Rendering for MenuGroup {
    fn render(&self, canvas: Canvas) {
        canvas.char(' ', 0, 0);
        canvas.text(&self.caption, 1, 0);
        canvas.char(' ', self.caption.len() as u16 + 1, 0);
    }
}

impl HasWidth for MenuChild {
    fn width(&self) -> size {
        10
    }
}

impl HasHeight for MenuChild {
    fn height(&self) -> size {
        1
    }
}

impl Rendering for MenuChild {
    fn render(&self, canvas: Canvas) {

    }
}
