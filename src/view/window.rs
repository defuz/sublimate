use toolkit::*;
use core::Core;
use view::menubar::Menubar;

pub struct Window {
    core: Core,
    menubar: Menubar
}

impl Window {
    pub fn new(core: Core) -> Window {
        let menubar = Menubar::new(&core);

        Window { core: core, menubar: menubar }
    }

    pub fn render(&self, canvas: Canvas) {
        self.menubar.render(&self.core, canvas);
    }
}
