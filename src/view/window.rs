use toolkit::*;
use core::Core;
use view::menubar::Menubar;
use view::context::ContextMenu;

#[derive(Debug)]
pub struct Window {
    core: Core,
    menubar: Menubar,
    context_menus: Vec<ContextMenu>
}

impl Window {
    pub fn new(core: Core) -> Window {
        let (menubar, menus) = Menubar::new(&core);

        Window {
            core: core,
            menubar: menubar,
            context_menus: menus
        }
    }

    pub fn render(&self, mut canvas: Canvas) {
        let ref menu = self.context_menus[0];
        menu.render(&self.core, canvas.cut_left(menu.width(&self.core)));
        // self.menubar.render(&self.core, canvas);
    }
}
