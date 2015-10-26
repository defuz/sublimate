use toolkit::*;
use core::Core;

use core::keymap::Key;

use view::menubar::Menubar;
use view::context::ContextMenu;
use view::event::OnKeypress;
use view::modal::ModalPosition;

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

    // pub fn open_modal_window(&mut self, id: &str, position: ModalPosition) {

    // }

    pub fn render(&self, mut canvas: Canvas) {
        // let ref menu = self.context_menus[0];
        // menu.render(&self.core, canvas.cut_left(menu.width(&self.core)));
        self.menubar.render(&self.core, canvas.cut_top(1));
    }
}

impl OnKeypress<()> for Window {
    fn on_keypress(&mut self, core: (), mut canvas: Canvas, key: Key) -> bool {
        self.menubar.on_keypress(&self.core, canvas.cut_top(1), key)
    }
}
