use toolkit::*;
use core::Core;

use core::keymap::Key;

use view::menubar::Menubar;
use view::context::ContextMenu;
use view::event::OnKeypress;
use view::modal::ModalManager;

#[derive(Debug)]
pub struct Window {
    pub core: Core,
    menubar: Menubar,
    // context_menus: Vec<ContextMenu>,
    modals: ModalManager,
}

#[derive(Debug)]
pub struct Context<'c> {
    pub core: &'c mut Core,
    pub modals: &'c mut ModalManager
}

impl Window {
    pub fn new(core: Core) -> Window {
        let (menubar, menus) = Menubar::new(&core);

        Window {
            core: core,
            menubar: menubar,
            modals: ModalManager::new(menus),
        }
    }

    // pub fn open_modal_window(&mut self, id: &str, position: ModalPosition) {

    // }

    // pub fn context(&'c self) -> Context<'c> {
    //     Context { core: &self.core, modals: &self.modals }
    // }

    pub fn on_keypress(&mut self, mut canvas: Canvas, key: Key) {
        let Window {ref mut core, ref mut menubar, ref mut modals} = *self;
        menubar.on_keypress(Context { core: core, modals: modals}, canvas.cut_top(1), key);
    }


    pub fn render(&self, mut canvas: Canvas) {
        // let ref menu = self.context_menus[0];
        // menu.render(&self.core, canvas.cut_left(menu.width(&self.core)));
        self.menubar.render(&self.core, canvas.cut_top(1));
        self.modals.update();
    }
}
