use toolkit::*;
use core::Core;

use core::bindings::Key;

use view::menubar::Menubar;
use view::editor::Editor;

#[derive(Debug)]
pub struct Window {
    core: Core,
    menubar: Menubar,
    editor: Editor
}

impl Window {
    pub fn new(core: Core) -> Window {
        let menubar = Menubar::new(core.package_repository.get_menu("default/Main.sublime-menu"));
        let editor = Editor::new(core.create_highlighter().unwrap(), ColorPalette::new(32, 255));
        Window {
            core: core,
            menubar: menubar,
            editor: editor
        }
    }

    pub fn on_keypress(&mut self, mut canvas: Canvas, key: Key) {
        self.menubar.on_keypress(&self.core, canvas.cut_top(1), key);
    }


    pub fn render(&self, mut canvas: Canvas) {
        self.menubar.view(&self.core).render(canvas.cut_top(1));
        self.editor.view(&self.core).render(canvas);
    }
}
