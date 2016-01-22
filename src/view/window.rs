use toolkit::*;
use core::Core;

use core::bindings::Key;

use view::menubar::Menubar;
use view::editor::Editor;
use view::sidebar::Sidebar;

#[derive(Debug)]
pub struct Window {
    core: Core,
    menubar: Menubar,
    editor: Editor,
    sidebar: Sidebar,
}

impl Window {
    pub fn new(core: Core) -> Window {
        let menubar = Menubar::new(&core);
        let sidebar = Sidebar::new(&core);
        let editor = Editor::new(&core);
        Window {
            core: core,
            menubar: menubar,
            sidebar: sidebar,
            editor: editor
        }
    }

    pub fn on_keypress(&mut self, mut canvas: Canvas, key: Key) {
        self.menubar.on_keypress(&self.core, canvas.cut_top(1), key);
    }


    pub fn render(&self, mut canvas: Canvas) {
        self.menubar.view(&self.core).render(canvas.cut_top(1));
        self.sidebar.view(&self.core).render(canvas.cut_left(30));
        self.editor.view(&self.core).render(canvas);
    }
}
